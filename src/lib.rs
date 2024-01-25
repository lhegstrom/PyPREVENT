use pyo3::prelude::*;
use heart_failure::*;
use ascvd::*;

mod heart_failure;
mod ascvd;

#[pymodule]
fn _ascvd(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(calculate_10_yr_heart_failure_rust, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_30_yr_heart_failure_rust, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_10_yr_ascvd_rust, m)?)?;
    m.add_function(wrap_pyfunction!(calculate_30_yr_ascvd_rust, m)?)?;
    Ok(())
}
