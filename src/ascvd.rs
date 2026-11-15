//! Atherosclerotic cardiovascular disease (ASCVD) equations and their Python bindings.

use crate::model_coefficients::{
    RiskModelCoefficients, FEMALE_10_YR_ASCVD, FEMALE_30_YR_ASCVD, MALE_10_YR_ASCVD,
    MALE_30_YR_ASCVD,
};
use crate::risk_calculation::{calculate_batch_risk, calculate_patient_risk};
use crate::risk_factors::{RiskFactors, RiskHorizon, Sex};
use numpy::PyReadonlyArrayDyn;
use pyo3::prelude::*;

fn select_ascvd_coefficients(sex: Sex, horizon: RiskHorizon) -> &'static RiskModelCoefficients {
    match (sex, horizon) {
        (Sex::Female, RiskHorizon::TenYear) => &FEMALE_10_YR_ASCVD,
        (Sex::Male, RiskHorizon::TenYear) => &MALE_10_YR_ASCVD,
        (Sex::Female, RiskHorizon::ThirtyYear) => &FEMALE_30_YR_ASCVD,
        (Sex::Male, RiskHorizon::ThirtyYear) => &MALE_30_YR_ASCVD,
    }
}

/// 10-year ASCVD risk percentage for one patient.
#[pyfunction]
pub fn calculate_10_yr_ascvd_rust(sex: &str, risk_factors: RiskFactors) -> PyResult<f64> {
    calculate_patient_risk(
        sex,
        &risk_factors,
        RiskHorizon::TenYear,
        select_ascvd_coefficients,
    )
}

/// 30-year ASCVD risk percentage for one patient.
#[pyfunction]
pub fn calculate_30_yr_ascvd_rust(sex: &str, risk_factors: RiskFactors) -> PyResult<f64> {
    calculate_patient_risk(
        sex,
        &risk_factors,
        RiskHorizon::ThirtyYear,
        select_ascvd_coefficients,
    )
}

/// 10-year ASCVD risk percentages for each row of an `(n, 11)` array.
#[pyfunction]
pub fn calculate_10_yr_ascvd_rust_parallel_np(
    py: Python<'_>,
    data: PyReadonlyArrayDyn<'_, f64>,
) -> PyResult<Py<PyAny>> {
    calculate_batch_risk(py, data, RiskHorizon::TenYear, select_ascvd_coefficients)
}

/// 30-year ASCVD risk percentages for each row of an `(n, 11)` array.
#[pyfunction]
pub fn calculate_30_yr_ascvd_rust_parallel_np(
    py: Python<'_>,
    data: PyReadonlyArrayDyn<'_, f64>,
) -> PyResult<Py<PyAny>> {
    calculate_batch_risk(py, data, RiskHorizon::ThirtyYear, select_ascvd_coefficients)
}
