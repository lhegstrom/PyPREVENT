# PyPREVENT :anatomical_heart:

###### 2024 updates from AHA on ASCVD (Atherosclerotic and Cardiovascular Disease), CVD (cardiovascular disease) and Heart Failure (HF)

<h1 align="center">
  <img src="https://github.com/lhegstrom/PyPREVENT/blob/main/images/PyPreventLogo.png?raw=true" width="100%">
  <br>
</h1>

![Python 3.10+](https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

## Introduction
The PyPREVENT Equations package offers 2024 updates from the [American Heart Association (AHA) on Atherosclerotic and
Cardiovascular Disease (ASCVD), Cardiovascular Disease (CVD), and Heart Failure
(HF)](https://professional.heart.org/en/guidelines-and-statements/prevent-calculator)[^1][^2]. It's a mixed Rust and Python
module, leveraging the speed of Rust for equation implementation and the flexibility of Python for ease of use.

## Installation
This checkout prepares **0.1.7**, which has not been published by this work.
The dependency and compatibility information below describes this version.

**Requirements:**
- Python 3.10 or newer. CI targets standard CPython 3.10–3.15 on Linux, Windows, and macOS.
- NumPy >=1.26.4,<3 and pandas >=2.2.3,<4 are installed automatically.
- Building from source requires Rust; the repository pins its tested toolchain in `rust-toolchain.toml`.


To install the currently published package from PyPI:
```bash
python -m pip install pyprevent
```

To install this 0.1.7 checkout instead, run `python -m pip install .` from the
repository root with Rust installed. Until 0.1.7 is released, a PyPI installation
does not include these maintenance changes.

## Build and compatibility status

The shared [distribution workflow](.github/workflows/windows_build.yml) builds
Linux, Windows, and macOS packages; its filename is historical. Each platform
below runs the same Python 3.10–3.15 matrix, for **24 wheel jobs**:

| Platform | Architecture | Runner | Wheel build |
|---|---|---|---|
| Linux | x86_64 | `ubuntu-latest` | manylinux2014 container |
| Windows | x86_64 | `windows-latest` | Native MSVC |
| macOS, Apple Silicon | arm64 | `macos-15` | Native Apple Silicon |
| macOS, Intel | x86_64 | `macos-15-intel` | Native Intel |

Every job uses Rust 1.92.0, Maturin 1.16.0, and the checked-in Cargo.lock, then
installs its wheel, runs tests outside the checkout, audits the installed
dependencies, and uploads a uniquely named artifact. A separate Ubuntu job
builds the source distribution, rebuilds a wheel from it, and tests that wheel.

Local Apple Silicon validation passed **315 tests on each of Python 3.10, 3.12,
3.13, 3.14, and 3.15**. Version 0.1.7 specifically built and passed the suite on
Python 3.14.10 and 3.15.2; both wheels and the source distribution passed Twine
validation. The hosted matrix, including Linux, Windows, and Intel macOS, has
**not yet run for these changes**. These are standard GIL-enabled CPython builds;
free-threaded Python, Linux ARM64, and Windows ARM64 are not in this matrix.

See [validation and benchmark results](docs/audit/IMPLEMENTATION.md) and the
[0.1.7 release notes](CHANGELOG.md). Local million-row benchmarks measured about
3x faster Rust computation and 16% lower complete batch-call time, compared with
the previous code on the same machine; results vary by hardware and workload.

## TL;DR

```python
import pyprevent

pyprevent.calculate_30_yr_ascvd_risk(
    sex="MALE",
    age=40,
    total_cholesterol=200,
    hdl_cholesterol=50,
    systolic_bp=120,
    has_diabetes=False,
    current_smoker=False,
    bmi=25,
    egfr=70,
    on_htn_meds=False,
    on_cholesterol_meds=False,
)
```

## Examples

[A longer, and more thorough example is located here.](example_notebooks/Example%20Notebook.ipynb)


## Implementation Status

| Formula                          | Status             |
|----------------------------------|--------------------|
| 10 yr ASCVD (individual)         | :white_check_mark: |
| 10 yr ASCVD (batch)              | :white_check_mark: |
| 30 yr ASCVD (individual)         | :white_check_mark: |
| 30 yr ASCVD (batch)              | :white_check_mark: |
| 10 yr Heart Failure (individual) | :white_check_mark: |
| 10 yr Heart Failure (batch)      | :white_check_mark: |
| 30 yr Heart Failure (individual) | :white_check_mark: |
| 30 yr Heart Failure (batch)      | :white_check_mark: |
| 10 yr CVD (individual)           | :white_check_mark: |
| 10 yr CVD (batch)                | :white_check_mark: |
| 30 yr CVD (individual)           | :white_check_mark: |
| 30 yr CVD (batch)                | :white_check_mark: |


## Program Structure

This is a mixed [Rust](https://www.rust-lang.org/) and Python module.

Rust implements the numerical equations and parallel batch computation. Python
provides the public API and pandas DataFrame conversion; Maturin compiles and
packages the extension.

The rust source code is located in the /src directory.
The individual functions are written in their respective files, and registered to the pyprevent._pyprevent Python module within the lib.rs file.

The python source is located in the /pyprevent directory.

Unit tests are in `tests/` and use pytest and Hypothesis.


[^1]: Khan SS, Matsushita K, Sang Y, et al. Development and Validation of the American Heart Association Predicting Risk of Cardiovascular Disease EVENTs (PREVENTTM) Equations. Circulation 2023. DOI: [10.1161/CIRCULATIONAHA.123.067626](https://www.ahajournals.org/doi/10.1161/CIRCULATIONAHA.123.067626)
[^2]: Khan SS, Coresh J, Pencina MJ, et al. Novel Prediction Equations for Absolute Risk Assessment of Total Cardiovascular Disease Incorporating Cardiovascular-Kidney-Metabolic Health: A Scientific Statement From the American Heart Association. Circulation 2023;148(24):1982-2004. DOI: [10.1161/CIR.0000000000001191](https://www.ahajournals.org/doi/10.1161/CIR.0000000000001191)


## Batch validation

Batch calls preserve row order and return `NaN` for invalid rows, with a summary
warning printed to stdout. Sex labels are case-insensitive `male` or `female`;
boolean fields accept booleans or numeric 0/1. Unknown or missing categories and
non-finite/out-of-range numeric inputs do not produce scores. Scalar calls retain
their existing exception behavior. Input DataFrames are not modified.

## Development and validation

Use a fresh virtual environment when upgrading from the old notebook-heavy
requirements file. Removing a package from requirements does not uninstall it
from an existing environment.

```bash
python -m venv venv
# Activate venv using the command appropriate for your shell.
python -m pip install -r requirements-build.txt
maturin develop --release --extras test
pytest
```

Optional notebook tools are available on **Python 3.11.5+** with
`python -m pip install '.[notebook]'`. On older Python versions that extra installs no
notebook tools: patched notebook dependencies require a newer interpreter.
The calculator itself continues to support Python 3.10.
The package uses Maturin exclusively; `setup.py install` is no longer supported.
`requirements.txt` installs the local package and its runtime dependencies.

The full audit lock includes runtime, test, notebook, and build dependencies:

```bash
uv pip compile pyproject.toml requirements-build.txt --extra test --extra notebook --python-version 3.10 --universal --generate-hashes --output-file requirements-audit.txt
pip-audit -r requirements-audit.txt --require-hashes --disable-pip
cargo audit --deny warnings
```

CI also audits the actual wheel test environments. The lock is a reproducible
maintenance snapshot; library users get the compatible ranges in `pyproject.toml`.

Run benchmarks against installed release wheels (not `maturin develop` debug
builds), using separate processes for different Rayon thread counts:

```bash
RAYON_NUM_THREADS=4 python benchmarks/batch.py --output /tmp/benchmark.json
```

## Publishing

Pushes to `main` build, test, and audit distributions without publishing.
Pull requests and pushes to other branches do not trigger builds. Manual runs
are accepted only on `main`. Release tags must point to a commit in the history
of `main`; a source-validation job enforces this before any build starts.
Only a published GitHub release with a matching `v<package-version>` tag can enter
the publishing job, after all checks succeed.

Before the next release, configure a PyPI Trusted Publisher for repository
`lhegstrom/PyPREVENT`, workflow `windows_build.yml`, and environment `pypi`.
Configure required reviewers on the GitHub `pypi` environment. The workflow uses
OIDC rather than the former `PYPI_API_TOKEN`; it will fail closed until Trusted
Publishing is configured. See the [PyPI setup guide](https://docs.pypi.org/trusted-publishers/adding-a-publisher/).

Version 0.1.7 is set in Cargo.toml, pyproject.toml, and Cargo.lock. After merging
the `0.1.7` branch into `main` and reviewing successful CI artifacts, a `v0.1.7`
GitHub release can publish the distributions. No release has been published by
these maintenance edits.
