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

Install the published package from PyPI:
```bash
python -m pip install pyprevent
```

To install from a local copy of this repository, run `python -m pip install .`
from the repository root. This requires Rust.

**Requirements:**
- Python 3.10 or later. The continuous integration (CI) workflow tests standard CPython 3.10 to 3.15 on Linux, Windows, and macOS.
- NumPy 1.26.4 or later (below 3) and pandas 2.2.3 or later (below 4). pip installs them automatically.
- To build from source, install the Rust toolchain. The `rust-toolchain.toml` file defines the tested version.

## Build and test matrix

The distribution workflow, [`windows_build.yml`](.github/workflows/windows_build.yml),
builds wheels for Linux, Windows, and macOS. Its filename is historical. Each
platform below runs the same Python 3.10 to 3.15 matrix. This creates **24 wheel jobs**.

| Platform | Architecture | Runner | Wheel build |
|---|---|---|---|
| Linux | x86_64 | `ubuntu-latest` | manylinux2014 container |
| Windows | x86_64 | `windows-latest` | Native MSVC |
| macOS, Apple Silicon | arm64 | `macos-15` | Native Apple Silicon |
| macOS, Intel | x86_64 | `macos-15-intel` | Native Intel |

Each wheel job:
1. Builds the wheel with Rust 1.92.0 and Maturin 1.16.0, using the committed `Cargo.lock`.
2. Installs the wheel with its test extras.
3. Runs the tests outside the checkout. The suite has 315 tests.
4. Runs `pip-audit` on the installed dependencies.
5. Uploads the wheel as an artifact.

A separate Ubuntu job builds the source distribution. It rebuilds a wheel from
that source distribution and runs the tests on the rebuilt wheel.

These are standard GIL-enabled CPython builds. Free-threaded Python, Linux ARM64,
and Windows ARM64 are not in the matrix.

For validation and benchmark results, see [docs/audit/IMPLEMENTATION.md](docs/audit/IMPLEMENTATION.md).
For changes by version, see the [changelog](CHANGELOG.md).

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

Pushes to `main` build, test, and audit the distributions. They do not publish.
Pull requests and pushes to other branches do not start these builds. Manual runs
work only on `main`. A source check requires the commit to be in the history of `main`.

Only a published GitHub release can start the publish job. The release tag must
be `v<version>` and must match the package version. All checks must pass first.

Before you publish, configure a PyPI Trusted Publisher with these values:
- Repository: `lhegstrom/PyPREVENT`
- Workflow: `windows_build.yml`
- Environment: `pypi`

Also configure required reviewers on the GitHub `pypi` environment. The workflow
uses OIDC, not the former `PYPI_API_TOKEN`. It fails until Trusted Publishing is
configured. See the [PyPI setup guide](https://docs.pypi.org/trusted-publishers/adding-a-publisher/).

To release a version, set the same version in `Cargo.toml`, `pyproject.toml`, and
`Cargo.lock`. Merge the change into `main`, review the CI artifacts, and then publish
a GitHub release with the tag `v<version>`.
