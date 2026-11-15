# Changelog

## 0.1.7 — unreleased

- Upgrade Rust bindings and dependencies to address known advisories.
- Declare NumPy/pandas runtime requirements and separate optional test/notebook tools.
- Reject invalid batch categories instead of silently calculating a female-model score.
- Reduce batch conversion and computation overhead while preserving valid-input results.
- Test standard CPython 3.10–3.15 in CI; locally verified 315 tests on Python 3.14 and 3.15.
- Build explicit Linux/Windows x86_64 and macOS Apple Silicon/Intel wheel targets.
- Restrict automatic builds to `main`; manual runs and releases must pass a main-source gate.
- Publish only intentional releases after tests/audits, using PyPI Trusted Publishing.
- Restructure the Rust code around typed `Sex`, `RiskHorizon`, and `RiskFactors` inputs, with
  one shared equation and coefficient constants. Results are bit-identical; clippy passes.
- Pass scalar inputs from Python to Rust as one named `risk_factors` mapping. Wrong input
  types still raise `TypeError`; the message now names the field.
- Rename `pyprevent/utils.py` to `pyprevent/batch_preparation.py` and move batch constants to
  `pyprevent/config.py`. Add a type stub for the native module.
- Remove a stray top-level `__init__.py` that made pytest treat the checkout as a package.

Calculator installations require Python 3.10+. Notebook extras require Python
3.11.5+. The redundant `setup.py install` path has been removed; install with pip
using the Maturin build backend.
