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

Calculator installations require Python 3.10+. Notebook extras require Python
3.11.5+. The redundant `setup.py install` path has been removed; install with pip
using the Maturin build backend.
