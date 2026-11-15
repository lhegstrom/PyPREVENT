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
from collections.abc import Callable
from concurrent.futures import ThreadPoolExecutor
from pathlib import Path
from typing import Any

import numpy as np
import pandas as pd

import pyprevent
from pyprevent import _pyprevent
from pyprevent.batch_preparation import _build_native_batch_array

# ============================================================
# Benchmark Settings
# ============================================================
RANDOM_SEED = 342
DEFAULT_ROW_COUNTS = [1, 10, 1000, 100000, 1000000]
DEFAULT_REPEAT_COUNT = 5
REFERENCE_ROW_COUNT = 10000
MAXIMUM_DIFFERENCE_TOLERANCE = 1e-13


def generate_random_patient_frame(row_count: int) -> pd.DataFrame:
    """Return in-range random patients; the fixed seed makes runs comparable."""
    random_generator = np.random.default_rng(RANDOM_SEED)
    return pd.DataFrame(
        {
            "sex": random_generator.choice(["male", "female"], row_count),
            "age": random_generator.uniform(30, 59, row_count),
            "total_cholesterol": random_generator.uniform(130, 320, row_count),
            "hdl_cholesterol": random_generator.uniform(20, 100, row_count),
            "systolic_bp": random_generator.uniform(90, 200, row_count),
            "has_diabetes": random_generator.choice([True, False], row_count),
            "current_smoker": random_generator.choice([True, False], row_count),
            "bmi": random_generator.uniform(18.5, 39.9, row_count),
            "egfr": random_generator.uniform(15, 140, row_count),
            "on_htn_meds": random_generator.choice([True, False], row_count),
            "on_cholesterol_meds": random_generator.choice([True, False], row_count),
        }
    )


def measure_median_runtime_ms(call: Callable[[], Any], repeat_count: int) -> float:
    """Run once to warm the native thread pool and imports, then time repeats."""
    call()
    runtimes_ms = []
    for _ in range(repeat_count):
        start_time = time.perf_counter()
        call()
        runtimes_ms.append((time.perf_counter() - start_time) * 1000)
    return statistics.median(runtimes_ms)


def measure_batch_timings(row_count: int, repeat_count: int) -> dict[str, float]:
    """Time conversion, native, strided, invalid, end-to-end, and threaded calls."""
    native_function = _pyprevent.calculate_10_yr_cvd_rust_parallel_np
    patient_frame = generate_random_patient_frame(row_count=row_count)
    native_array = _build_native_batch_array(df=patient_frame)
    # Twice the storage, same shape and values, non-contiguous columns.
    backing_array = np.empty((row_count, 22))
    backing_array[:, ::2] = native_array
    strided_array = backing_array[:, ::2]
    invalid_array = native_array.copy()
    invalid_array[::10, 1] = np.nan
    with ThreadPoolExecutor(max_workers=2) as pool:
        return {
            "rows": row_count,
            "conversion_ms": measure_median_runtime_ms(
                call=lambda: _build_native_batch_array(df=patient_frame),
                repeat_count=repeat_count,
            ),
            "native_ms": measure_median_runtime_ms(
                call=lambda: native_function(data=native_array),
                repeat_count=repeat_count,
            ),
            "strided_ms": measure_median_runtime_ms(
                call=lambda: native_function(data=strided_array),
                repeat_count=repeat_count,
            ),
            "invalid_ms": measure_median_runtime_ms(
                call=lambda: native_function(data=invalid_array),
                repeat_count=repeat_count,
            ),
            "end_to_end_ms": measure_median_runtime_ms(
                call=lambda: pyprevent.batch_calculate_10_yr_cvd_risk(df=patient_frame),
                repeat_count=repeat_count,
            ),
            "two_calls_sequential_ms": measure_median_runtime_ms(
                call=lambda: (
                    native_function(data=native_array),
                    native_function(data=native_array),
                ),
                repeat_count=repeat_count,
            ),
            "two_calls_threaded_ms": measure_median_runtime_ms(
                call=lambda: list(
                    pool.map(native_function, [native_array, native_array])
                ),
                repeat_count=repeat_count,
            ),
        }


def measure_peak_memory_mib() -> float | None:
    """Return this process's peak resident memory, or None where unsupported."""
    try:
        import resource
    except ImportError:
        return None
    peak_resident_memory = resource.getrusage(resource.RUSAGE_SELF).ru_maxrss
    # macOS reports bytes; Linux reports KiB.
    return peak_resident_memory / (1024**2 if platform.system() == "Darwin" else 1024)


def calculate_reference_scores() -> dict[str, np.ndarray]:
    """Score a fixed sample with every native batch function, keyed like '10_cvd'."""
    native_batch_array = _build_native_batch_array(
        df=generate_random_patient_frame(row_count=REFERENCE_ROW_COUNT)
    )
    return {
        f"{years}_{model}": getattr(
            _pyprevent, f"calculate_{years}_yr_{model}_rust_parallel_np"
        )(data=native_batch_array)
        for years in (10, 30)
        for model in ("ascvd", "cvd", "hf")
    }


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--sizes", type=int, nargs="+", default=DEFAULT_ROW_COUNTS)
    parser.add_argument("--repeats", type=int, default=DEFAULT_REPEAT_COUNT)
    parser.add_argument("--save-reference", type=Path)
    parser.add_argument("--reference", type=Path)
    arguments = parser.parse_args()
    benchmark_result: dict[str, Any] = {
        "python": platform.python_version(),
        "platform": platform.platform(),
        "numpy": np.__version__,
        "pandas": pd.__version__,
        "package": pyprevent.__file__,
        "rayon_threads": os.getenv("RAYON_NUM_THREADS", "default"),
        "repeats": arguments.repeats,
        "measurements": [],
    }
    for row_count in arguments.sizes:
        batch_timings = measure_batch_timings(
            row_count=row_count, repeat_count=arguments.repeats
        )
        benchmark_result["measurements"].append(batch_timings)
        print(batch_timings, flush=True)
    benchmark_result["process_peak_rss_mib"] = measure_peak_memory_mib()

    reference_scores = calculate_reference_scores()
    if arguments.save_reference:
        # The numpy stub cannot rule out an "allow_pickle" key; keys are like "10_cvd".
        np.savez(arguments.save_reference, **reference_scores)  # type: ignore[arg-type]
    if arguments.reference:
        with np.load(arguments.reference) as saved_scores:
            benchmark_result["max_absolute_error_percentage_points"] = max(
                float(np.max(np.abs(scores - saved_scores[key])))
                for key, scores in reference_scores.items()
            )
            for key, scores in reference_scores.items():
                np.testing.assert_allclose(
                    scores,
                    saved_scores[key],
                    rtol=MAXIMUM_DIFFERENCE_TOLERANCE,
                    atol=MAXIMUM_DIFFERENCE_TOLERANCE,
                )
    arguments.output.write_text(json.dumps(benchmark_result, indent=2) + "\n")


if __name__ == "__main__":
    main()
