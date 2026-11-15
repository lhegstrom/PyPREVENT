//! Total cardiovascular disease (CVD) equations and their Python bindings.

use crate::model_coefficients::{
    RiskModelCoefficients, FEMALE_10_YR_CVD, FEMALE_30_YR_CVD, MALE_10_YR_CVD, MALE_30_YR_CVD,
};
use crate::risk_calculation::{calculate_batch_risk, calculate_patient_risk};
use crate::risk_factors::{RiskFactors, RiskHorizon, Sex};
use numpy::PyReadonlyArrayDyn;
use pyo3::prelude::*;

fn select_cvd_coefficients(sex: Sex, horizon: RiskHorizon) -> &'static RiskModelCoefficients {
    match (sex, horizon) {
        (Sex::Female, RiskHorizon::TenYear) => &FEMALE_10_YR_CVD,
        (Sex::Male, RiskHorizon::TenYear) => &MALE_10_YR_CVD,
        (Sex::Female, RiskHorizon::ThirtyYear) => &FEMALE_30_YR_CVD,
        (Sex::Male, RiskHorizon::ThirtyYear) => &MALE_30_YR_CVD,
    }
}

/// 10-year CVD risk percentage for one patient.
#[pyfunction]
pub fn calculate_10_yr_cvd_rust(sex: &str, risk_factors: RiskFactors) -> PyResult<f64> {
    calculate_patient_risk(
        sex,
        &risk_factors,
        RiskHorizon::TenYear,
        select_cvd_coefficients,
    )
}

/// 30-year CVD risk percentage for one patient.
#[pyfunction]
pub fn calculate_30_yr_cvd_rust(sex: &str, risk_factors: RiskFactors) -> PyResult<f64> {
    calculate_patient_risk(
        sex,
        &risk_factors,
        RiskHorizon::ThirtyYear,
        select_cvd_coefficients,
    )
}

/// 10-year CVD risk percentages for each row of an `(n, 11)` array.
#[pyfunction]
pub fn calculate_10_yr_cvd_rust_parallel_np(
    py: Python<'_>,
    data: PyReadonlyArrayDyn<'_, f64>,
) -> PyResult<Py<PyAny>> {
    calculate_batch_risk(py, data, RiskHorizon::TenYear, select_cvd_coefficients)
}

/// 30-year CVD risk percentages for each row of an `(n, 11)` array.
#[pyfunction]
pub fn calculate_30_yr_cvd_rust_parallel_np(
    py: Python<'_>,
    data: PyReadonlyArrayDyn<'_, f64>,
) -> PyResult<Py<PyAny>> {
    calculate_batch_risk(py, data, RiskHorizon::ThirtyYear, select_cvd_coefficients)
}
