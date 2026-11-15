# Maintenance changes and validation

Implemented November 15, 2026 against main at `89e99bb`, then prepared as
version **0.1.7** on branch **`0.1.7`**. No GitHub release or PyPI upload was
performed. The original audit/benchmark evidence remains a historical
snapshot from before the version bump.

## Changes

- Updated PyO3 to 0.30.1, rust-numpy to 0.30.0, Rayon to 1.12.0 in the lockfile,
  and crossbeam-epoch to 0.9.21. Migrated module/array bindings to the current API.
  Rust 1.92.0 is pinned for reproducible builds; the local global Rust default was
  not changed.
- Consolidated packaging under Maturin; removed the custom setup.py installer and
  conflicting pytest.ini. Declared NumPy >=1.26.4,<3 and pandas >=2.2.3,<4, with
  separate test/notebook extras and build requirements. The runtime requirements
  file now installs the local package rather than an entire notebook environment.
- Calculator support starts at Python 3.10. Notebook extras require Python
  **3.11.5+**; they install no notebook tools on older interpreters. Universal
  dependency resolution otherwise selected vulnerable older Soup Sieve/Tornado
  versions to satisfy Python 3.10. Explicit patched floors and environment
  markers avoid this downgrade without removing calculator support for 3.10.
- Invalid batch categories, missing values, and numeric errors yield NaN in the
  affected rows, preserving row order and the existing summary-warning behavior.
  Unknown keyword mappings and duplicate/missing column names raise explicit
  errors. Valid scalar APIs and coefficients are unchanged.
- DataFrame conversion allocates one float64 result matrix and avoids a full
  DataFrame copy. The native call snapshots input into owned Rust memory before
  releasing the GIL; it does not read borrowed NumPy memory while detached.
  Contiguous and strided inputs preserve logical row order. Batches below 256
  rows run sequentially to avoid Rayon scheduling overhead.
- Replaced per-patient temporary heap vectors with stack arrays, avoided sex
  string allocations, and calculated each exponential once with `exp()`.
- Added invalid-input, nullable-column, custom-column, shape, striding, ordering,
  concurrency, and all-six-model parity tests. Converted the existing assertion
  helpers from Slash to pytest and added meaningful Hypothesis output checks.
- Corrected two stale female 10-year heart-failure test expectations. The
  unchanged baseline wheel reproduces their failures; upstream commit `d0185be`
  had changed the diabetes coefficient from 1.0 to 1.038346 without updating
  those fixtures. The regression value is now 4.3114057073104215 for that patient.
  No coefficients were changed in this work; these fixtures establish regression
  behavior and do not constitute a new independent clinical validation.
- Reworked CI to build/install/test wheels for standard CPython 3.10–3.15 on
  Linux, Windows, and macOS; rebuild/test the sdist; audit actual test environments
  and the complete maintenance lock; and check Rust advisories and formatting.
  Actions are pinned to verified commit hashes, with Dependabot updates enabled.
  Publication runs only for published releases with matching versions/tags after
  successful checks, using an OIDC publisher and a `pypi` environment.

## Security evidence

- `pip-audit` checked 122 selected package/version records from the complete
  maintenance lock in the local Python 3.12/macOS environment: **zero findings**.
  See [python-audit-after.json](python-audit-after.json).
- A supplemental PyPI version-advisory scan checked **134 distinct package/version
  pairs across all markers**, including other Python versions and operating
  systems: **zero findings**. See
  [python-audit-after-all-platforms.json](python-audit-after-all-platforms.json).
  This supplements environment resolution; it does not simulate other platforms.
- OSV checked all 30 registry crates in the updated lock: **zero findings**.
  See [rust-audit-after.json](rust-audit-after.json).
- `cargo-audit 0.23.0 audit --deny warnings` returned **zero vulnerabilities and
  zero advisory warnings**. See [cargo-audit-after.json](cargo-audit-after.json).

These are known-advisory checks at the time of the review, not a guarantee that
dependencies have no undiscovered defects. The original audit JSON and REVIEW.md
are retained as historical baseline evidence.

## Tests and packaging

The 315-test suite passes against installed packages on this Apple Silicon Mac:

| Python | NumPy | pandas | Installation |
|---|---|---|---|
| 3.10.13 | 1.26.4 | 2.2.3 | Clean source install at declared runtime floors |
| 3.12.3 | 1.26.4 | 2.2.3 | Built release wheel |
| 3.13.7 | 2.5.4 | 3.1.1 | Clean source-distribution install |
| 3.14.10 | 2.5.4 | 3.1.1 | Clean 0.1.7 source build/install |
| 3.15.2 | 2.5.4 | 3.1.1 | Clean 0.1.7 source build/install |

Tests run from outside the source checkout to exercise the installed extension.
The final wheel and sdist both pass `twine check`; wheel metadata declares the
runtime requirements and conditional extras correctly. A fresh install of the
final sdist also passes all 315 tests on Python 3.13. No development environment
or build-cache files were found in the distributions.
Rust formatting and Python lint checks pass (Ruff excludes the existing
mixed-case repository-name warning, N999). Clippy completes with the existing
13 `too_many_arguments` warnings associated with the calculation API; no new lint
category was reported. Workflow YAML was parsed locally; the full GitHub-hosted
matrix has not been run and remains a release gate.

## Performance evidence

Both baseline and updated wheels used Rust 1.92.0 release builds, CPython 3.12.3,
NumPy 1.26.4, and pandas 2.2.3 on the same machine. Each number is the median of
seven warm runs. The harness measures conversion, native compute, strided arrays,
10% invalid numeric rows, end-to-end calls, and two sequential/concurrent callers.

Four Rayon threads:

| Patients | End-to-end before (ms) | After (ms) | Native before (ms) | After (ms) |
|---:|---:|---:|---:|---:|
| 1 | 0.552 | 0.217 | 0.0008 | 0.0004 |
| 10 | 0.612 | 0.221 | 0.0174 | 0.0008 |
| 1,000 | 0.766 | 0.392 | 0.0714 | 0.0200 |
| 100,000 | 16.05 | 10.55 | 4.81 | 1.24 |
| 1,000,000 | 144.59 | 121.70 | 35.53 | 11.98 |

At one million rows this is about **3x faster native computation** and **16% lower
end-to-end latency**. Strided native input also improved (44.67 to 19.49 ms), so
the contiguous fast path does not come at the expense of the fallback. Whole
benchmark-process peak RSS fell from 878 to 730 MiB; that includes all benchmark
input buffers, not just library allocations. With one Rayon thread, million-row
end-to-end latency fell from 190.86 to 148.06 ms and native time from 77.08 to
37.09 ms. Results depend on hardware and data; small sub-millisecond numbers are
particularly sensitive to scheduling noise.

All six models were compared on 10,000 deterministic valid patients each. The
maximum difference across 60,000 scores was **2.842170943040401e-14 percentage
points**, within the 1e-13 relative/absolute regression tolerances.

Raw results: [before, four threads](benchmark-before-4.json),
[after, four threads](benchmark-after-4.json),
[before, one thread](benchmark-before-1.json),
[after, one thread](benchmark-after-1.json).
Reproduce using `benchmarks/batch.py` and separate installed release wheels.

## Release follow-up and deferred API work

Configure the PyPI Trusted Publisher and required reviewers on the GitHub `pypi`
environment before publishing. These remote settings were not changed. Version 0.1.7 is synchronized across
Cargo.toml, pyproject.toml, and Cargo.lock. Merge branch `0.1.7` into `main`, run
the hosted matrix, and review artifacts before creating release `v0.1.7`.

The optional combined six-model API remains deferred. The existing API now has
measured improvements; adding a new public result shape and shared calculation
path merits a workload-specific benchmark and separate API review. No combined
API or free-threaded-Python support is claimed by this change.

## Main-only workflow and Python 3.15 follow-up

The repository has one Actions workflow, `windows_build.yml`. Its push trigger
is restricted to `main` and its PR trigger has been removed. A source gate skips
manual runs selected from other refs and requires every build/audit job to wait
for verification. Releases must point to a commit reachable from `origin/main`;
an isolated Git fixture verified acceptance of main commits and rejection of
unmerged commits. Ordinary tag pushes do not trigger the workflow. Workflow
trigger/matrix/dependency assertions and YAML parsing pass locally.

The wheel matrix now includes Python 3.15 on all three operating systems, without
`continue-on-error`. PyO3 0.30.1 already supports Python 3.15 final; see its
[changelog](https://pyo3.rs/v0.30.1/changelog). The manylinux image family used in
CI lists CPython 3.14 and 3.15 in its [supported image contents](https://github.com/pypa/manylinux#image-content).
Local tests used standard GIL-enabled CPython, not free-threaded builds. The
Linux/Windows jobs still need to run on GitHub after merging to main; local macOS
success is not a claim that those remote jobs have already passed.

Built version 0.1.7 wheels for CPython 3.14 and 3.15 plus an sdist; all three
passed `twine check`. Manifest, lockfile, wheel version, and Python 3.15 classifier
consistency checks also passed. These changes were prepared on branch `0.1.7`.

The platform matrix was subsequently made explicit: Linux x86_64 (manylinux2014),
Windows x86_64 (MSVC), macOS arm64 (`macos-15`), and macOS x86_64
(`macos-15-intel`). All four targets use CPython 3.10–3.15, giving 24 wheel jobs
with unique artifact names. Python and Rust target architectures are specified
together, and every wheel is installed/tested on its native runner. The selected
Mac runner labels follow the [GitHub runner reference](https://docs.github.com/en/actions/how-tos/write-workflows/choose-where-workflows-run/choose-the-runner-for-a-job).
Local YAML checks verified the platform/architecture mappings, six Python
versions, main-only triggers, source gates, and artifact-name uniqueness. This
configuration/documentation update does not imply a hosted CI run has occurred.
