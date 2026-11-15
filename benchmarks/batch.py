"""Benchmark an installed release wheel, not an editable source checkout.

Run each variant in a fresh process with RAYON_NUM_THREADS set before startup.
Example: python benchmarks/batch.py --output /tmp/baseline.json
Use --save-reference /tmp/reference.npz on the old wheel and --reference on the new one.
"""

import argparse
import json
import os
import platform
import statistics
import time
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path

import numpy as np
import pandas as pd

import pyprevent
from pyprevent import _pyprevent
from pyprevent.utils import _prepare_df_for_batch


def patients(size):
    rng = np.random.default_rng(342)
    return pd.DataFrame(
        {
            "sex": rng.choice(["male", "female"], size),
            "age": rng.uniform(30, 59, size),
            "total_cholesterol": rng.uniform(130, 320, size),
            "hdl_cholesterol": rng.uniform(20, 100, size),
            "systolic_bp": rng.uniform(90, 200, size),
            "has_diabetes": rng.choice([True, False], size),
            "current_smoker": rng.choice([True, False], size),
            "bmi": rng.uniform(18.5, 39.9, size),
            "egfr": rng.uniform(15, 140, size),
            "on_htn_meds": rng.choice([True, False], size),
            "on_cholesterol_meds": rng.choice([True, False], size),
        }
    )


def timed(call, repeats):
    call()  # Warm native thread pool and imports.
    times = []
    for _ in range(repeats):
        start = time.perf_counter()
        call()
        times.append((time.perf_counter() - start) * 1000)
    return statistics.median(times)


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument(
        "--sizes", type=int, nargs="+", default=[1, 10, 1000, 100000, 1000000]
    )
    parser.add_argument("--repeats", type=int, default=5)
    parser.add_argument("--save-reference", type=Path)
    parser.add_argument("--reference", type=Path)
    args = parser.parse_args()
    result = {
        "python": platform.python_version(),
        "platform": platform.platform(),
        "numpy": np.__version__,
        "pandas": pd.__version__,
        "package": pyprevent.__file__,
        "rayon_threads": os.getenv("RAYON_NUM_THREADS", "default"),
        "repeats": args.repeats,
        "measurements": [],
    }
    native = _pyprevent.calculate_10_yr_cvd_rust_parallel_np
    for size in args.sizes:
        frame = patients(size)
        array = _prepare_df_for_batch(frame)
        # Twice the storage, same shape and values, non-contiguous columns.
        backing = np.empty((size, 22))
        backing[:, ::2] = array
        strided = backing[:, ::2]
        invalid = array.copy()
        invalid[::10, 1] = np.nan
        with ThreadPoolExecutor(max_workers=2) as pool:
            row = {
                "rows": size,
                "conversion_ms": timed(
                    lambda frame=frame: _prepare_df_for_batch(frame), args.repeats
                ),
                "native_ms": timed(lambda array=array: native(array), args.repeats),
                "strided_ms": timed(
                    lambda strided=strided: native(strided), args.repeats
                ),
                "invalid_ms": timed(
                    lambda invalid=invalid: native(invalid), args.repeats
                ),
                "end_to_end_ms": timed(
                    lambda frame=frame: pyprevent.batch_calculate_10_yr_cvd_risk(frame),
                    args.repeats,
                ),
                "two_calls_sequential_ms": timed(
                    lambda array=array: (native(array), native(array)), args.repeats
                ),
                "two_calls_threaded_ms": timed(
                    lambda pool=pool, array=array: list(
                        pool.map(native, [array, array])
                    ),
                    args.repeats,
                ),
            }
        result["measurements"].append(row)
        print(row, flush=True)
    try:
        import resource

        peak = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
        result["process_peak_rss_mib"] = peak / (
            1024**2 if platform.system() == "Darwin" else 1024
        )
    except ImportError:
        result["process_peak_rss_mib"] = None
    sample = _prepare_df_for_batch(patients(10000))
    scores = {}
    for years in (10, 30):
        for model in ("ascvd", "cvd", "hf"):
            key = f"{years}_{model}"
            scores[key] = getattr(
                _pyprevent, f"calculate_{years}_yr_{model}_rust_parallel_np"
            )(sample)
    if args.save_reference:
        np.savez(args.save_reference, **scores)
    if args.reference:
        with np.load(args.reference) as reference:
            result["max_absolute_error_percentage_points"] = max(
                float(np.max(np.abs(scores[key] - reference[key]))) for key in scores
            )
            for key, values in scores.items():
                np.testing.assert_allclose(
                    values, reference[key], rtol=1e-13, atol=1e-13
                )
    args.output.write_text(json.dumps(result, indent=2) + "\n")


if __name__ == "__main__":
    main()
