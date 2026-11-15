# Dependency and performance review

Historical scan and approved plan. See [IMPLEMENTATION.md](IMPLEMENTATION.md)
for the subsequent changes, updated audits, tests, and measured performance.

Reviewed November 15, 2026, at commit `89e99bb40982f5f3c4c281950210d66d4f96de33` (main).
This is a scan and proposed plan only. No application, dependency, or workflow changes were made.

## Scope and results

- Python: `pip-audit -r requirements.txt --no-deps --disable-pip --format json` checked all 114 pinned packages against current PyPI advisories, with no skipped packages. It reported 175 records across 19 packages. Merging records with overlapping IDs/aliases within each package gives **109 distinct package/advisory pairs**. Raw output: [python-audit.json](python-audit.json). Counts are advisory matches, not demonstrated exploits.
- Rust: queried OSV for all 47 registry packages in the checked-in Cargo.lock. Raw output: [rust-audit.json](rust-audit.json). Six returned records reduce to four issues; one is excluded after checking RustSec's more precise affected-version range. **Three applicable advisories across two crates remain.** This was an OSV lockfile scan, not a cargo-audit run; cargo-audit is not installed. The query script is [scan_rust_osv.py](scan_rust_osv.py), run from the repository root, and writes `/tmp/pyprevent-rust-audit.json`.
- Reviewed Python/Rust calculation and conversion paths, packaging, tests, and the build/publish workflow. No exploit testing, fresh wheel build, regression suite, or performance benchmarks were run. Performance suggestions below are hypotheses to measure, not claimed speedups.
- The lockfile still labels the root crate 0.1.5 while the manifests declare 0.1.6. It is a stale build snapshot, not proof of the precise crate versions inside published wheels. Audit resolved dependencies again after updating it.
- PyPI's `https://pypi.org/pypi/pyprevent/json` reports 0.1.6 with both `requires_dist` and `requires_python` null. Published wheels were not independently inspected. The Python scan covers the repository's pinned environment, not a claim that every downstream installation contains those packages. Newly resolved transitive dependencies absent from requirements.txt were not audited.

## Rust advisory triage

| Locked dependency | Advisory | Fix floor and relevance |
|---|---|---|
| pyo3 0.20.2 | [RUSTSEC-2025-0020](https://rustsec.org/advisories/RUSTSEC-2025-0020.html) | Fixed in 0.24.1. Out-of-bounds read through `PyString::from_object`; no direct invocation found in project source. |
| pyo3 0.20.2 | [RUSTSEC-2026-0177](https://rustsec.org/advisories/RUSTSEC-2026-0177.html) | Fixed in 0.29.0. Missing Sync bound on `PyCFunction::new_closure`; no direct invocation found in project source. |
| crossbeam-epoch 0.9.18 | [RUSTSEC-2026-0204](https://rustsec.org/advisories/RUSTSEC-2026-0204.html) | Fixed in 0.9.20. Invalid pointer dereference during pointer formatting. Transitive Rayon dependency; no corresponding pointer-formatting path identified in project source. |

An additional OSV/GitHub result, GHSA-36hh-v3qg-5jq4, broadly matches PyO3 <0.29.0. [RustSec RUSTSEC-2026-0176](https://rustsec.org/advisories/RUSTSEC-2026-0176.html) explicitly marks <0.24.0 unaffected. Do not count this as affecting locked PyO3 0.20.2. Avoid introducing it by stopping the upgrade at 0.24.1.

Proposed Rust target: PyO3 >=0.29.0 with a compatible rust-numpy release, plus crossbeam-epoch >=0.9.20 through an updated lockfile. Choose the exact supported release pair during implementation and re-audit it. PyO3's 0.x API changes require binding migrations; this is not a blind version replacement. Lack of a direct call does not prove a dependency advisory is unreachable through all generated or third-party code.

## Packaging, release, and correctness findings

1. **High priority: invalid batch categories produce plausible results.** `pyprevent/utils.py:49` maps unknown sex labels to NaN; `src/utils.rs:182` treats anything other than 1.0 as female. The numeric validator never validates that field, so otherwise valid rows receive a female-model score without an invalid-row warning. Numeric boolean columns also accept values such as 2 or -1 as true; direct native-array NaNs become true via `!= 0.0`. Define invalid-row behavior and validate categorical fields before conversion.
2. **High priority: publication is not restricted to releases.** `.github/workflows/windows_build.yml` triggers on all pushes and pull requests, and every build job has an unconditional Twine upload using the PyPI token. This can publish unreviewed code whenever credentials are available. Fork PRs normally lack secrets; this is not a claim that all external PRs can steal the token. Separate build/test from a protected, release-only publishing job, preferably using PyPI Trusted Publishing. Do not publish during remediation validation.
3. **Runtime dependency metadata is missing.** `pyproject.toml` declares neither NumPy nor pandas, although importing the package imports both. `setup.py` instead installs the entire 114-package requirements list using a separate custom install/build path. Consolidate packaging under Maturin, explicitly declare runtime requirements, and separate notebook/build/test extras. This reduces installation size and unnecessary security exposure; deleting development dependencies alone does not patch environments where they are still used.
4. **Tests do not gate release builds.** The workflow builds/uploads without running tests. Test dependencies `slash`, `hypothesis`, and a test runner are not declared in requirements.txt. Both testpaths settings point at nonexistent directories (`ascvd/tests` and `PyPrevent/tests`), while tests live at `tests/`. Hypothesis tests call functions but do not assert output properties; they still catch exceptions, but not incorrect numerical results.
5. **Compatibility policy needs alignment.** CI attempts Python versions through 3.14 while the Rust bindings and several Python pins date to early 2024. Define supported Python/NumPy versions, modernize the binding pair, and prove support by installing and testing built wheels on the actual matrix. Include macOS if it is a supported distribution target. Do not claim compatibility based only on a matrix entry.

## Enhancement plan — not implemented

Implement as separate, reviewable changes in this order:

1. **Establish correctness and release protection.** Add regression coverage for invalid/missing sex, categorical booleans, empty and non-contiguous arrays, and scalar/batch parity for all six models. Preserve valid-input results and the existing percentage scale. Repair test discovery and declare test dependencies. Gate publishing on successful wheel installation and tests, restrict publication to intentional releases, and keep coefficients unchanged.
2. **Remediate dependencies and packaging.** Upgrade PyO3/rust-numpy together; update the lockfile and compatible transitive crates. Declare NumPy/pandas runtime ranges, split development/notebook dependencies, and remove the redundant custom setup.py installation path once clean wheel/sdist installation is verified. Resolve and audit the complete resulting environments. Use the advisory floors below as inputs, not as a guaranteed mutually compatible requirements file. Add automated Python/Rust audits and dependency-update PRs.
3. **Create a performance baseline.** Benchmark release builds at 1, 10, 1,000, 100,000, and 1,000,000 rows; measure DataFrame conversion and native compute separately, plus peak memory. Include contiguous/strided input, invalid-row mixtures, one versus multiple Python caller threads, and Rayon thread counts. Compare every optimization against this baseline and numerical tolerances before merging.
4. **Reduce per-patient work.** In `src/utils.rs:112`, replace the temporary `vec!` of terms with a stack array or ordered accumulation, preserving summation order. Replace repeated `E.powf(calculation)` expressions with one `calculation.exp()` and reuse the value. Avoid lowercasing/allocating a sex string for every batch row by selecting validated typed categories/coefficients once. Compiler optimizations may already eliminate some costs; retain changes only if benchmarks or clarity justify them.
5. **Reduce batch memory and improve concurrency.** `pyprevent/utils.py:45-60` copies/reformats a DataFrame and creates a float64 array; `src/utils.rs:177-195` then materializes another full row vector. Prototype one deliberate contiguous conversion and avoid unnecessary intermediate copies. Release the GIL around computation on owned Rust data using the upgraded PyO3 API, then reacquire it to create the NumPy output. Rayon already runs Rust workers concurrently today; releasing the GIL lets other Python threads progress. Do not simply detach while borrowing externally mutable NumPy memory. See [PyO3 parallelism guidance](https://pyo3.rs/v0.29.0/parallelism.html).
6. **Tune small batches and repeated models.** Benchmark a sequential cutoff to avoid Rayon dispatch overhead for small inputs. Consider an optional combined six-model batch API that validates/converts a patient once and shares transformed covariates. Keep existing APIs working and measure total throughput/memory before adding this API.

Acceptance: no newly reported dependency advisories without documented triage; reproducible wheel/sdist installs; passing valid-input parity and invalid-input tests; no coefficient changes; publishing disabled in PR validation; measured performance gains without material numerical drift or memory regressions. Release/version changes require an intentional later release step.

## Python scan detail

The table below merges duplicate IDs/aliases per package. Fix versions are the highest version listed among this scan's advisories for that package; they are **candidate advisory floors, not tested upgrade pins or a dependency-resolution result**. All IDs, descriptions, and individual fix ranges are retained in the JSON. Most affected packages belong to notebook, build, or developer tooling rather than the calculator's direct NumPy/pandas runtime imports.

| Package | Pinned version | Distinct advisory matches | Candidate fix floor |
|---|---|---:|---|
| anyio | 4.2.0 | 2 | 4.14.2 |
| black | 23.3.0 | 4 | 26.10.1 |
| bleach | 6.1.0 | 2 | 6.4.0 |
| certifi | 2023.7.22 | 2 | 2026.10.18 |
| idna | 3.6 | 2 | 3.15 |
| jinja2 | 3.1.3 | 4 | 3.1.6 |
| jupyter-core | 5.7.1 | 1 | 5.8.1 |
| jupyter-server | 2.12.5 | 9 | 2.21.0 |
| jupyterlab | 4.0.11 | 12 | 4.6.4 |
| mistune | 3.0.2 | 17 | 3.3.0 |
| nbconvert | 7.14.2 | 4 | 7.17.1 |
| notebook | 7.0.7 | 3 | 7.5.6 |
| pygments | 2.17.2 | 1 | 2.20.0 |
| requests | 2.31.0 | 3 | 2.33.0 |
| setuptools | 65.6.3 | 4 | 84.1.0 |
| soupsieve | 2.5 | 8 | 4.0.0 |
| tornado | 6.4 | 22 | 6.6.1 |
| tqdm | 4.65.0 | 1 | 4.66.3 |
| urllib3 | 2.1.0 | 8 | 2.8.0 |
