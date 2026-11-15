//! Native PREVENT equations, exposed to Python as `pyprevent._pyprevent`.

use ascvd::*;
use cvd::*;
use heart_failure::*;
use pyo3::prelude::*;

mod ascvd;
mod config;
mod cvd;
mod heart_failure;
mod model_coefficients;
mod risk_calculation;
mod risk_equation;
mod risk_factors;

#[pymodule]
fn _pyprevent(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // single patients
    m.add_function(wrap_pyfunction!(calculate_10_yr_heart_failure_rust, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_30_yr_heart_failure_rust, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_10_yr_ascvd_rust, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_30_yr_ascvd_rust, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_10_yr_cvd_rust, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_30_yr_cvd_rust, m)?)?;
    // batch patients
    m.add_function(wrap_pyfunction!(calculate_10_yr_ascvd_rust_parallel_np, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_30_yr_ascvd_rust_parallel_np, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_10_yr_cvd_rust_parallel_np, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_30_yr_cvd_rust_parallel_np, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_10_yr_hf_rust_parallel_np, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_30_yr_hf_rust_parallel_np, m)?)?;
    Ok(())
}
