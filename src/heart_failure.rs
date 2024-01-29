use crate::covariates::Covariates;
use crate::utils::{calculate_risk_rust_parallel_np, common_calculation, validate_input};
use numpy::PyReadonlyArrayDyn;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::f64;
use std::f64::consts::E;

pub fn calculate_10_yr_heart_failure_risk(
    sex: &str,
    age: f64,
    total_cholesterol: f64,
    hdl_cholesterol: f64,
    systolic_blood_pressure: f64,
    has_diabetes: bool,
    is_current_smoker: bool,
    body_mass_index: f64,
    estimated_glomerular_filtration_rate: f64,
    on_hypertension_meds: bool,
    _cholesterol_treated: bool,
) -> Result<f64, String> {
    validate_input(
        age,
        total_cholesterol,
        hdl_cholesterol,
        systolic_blood_pressure,
        body_mass_index,
        estimated_glomerular_filtration_rate,
        true,
    )?;

    let cholesterol_difference = total_cholesterol - hdl_cholesterol;
    let age_factor = (age - 55.0) / 10.0;
    let age_squared = age_factor.powi(2);

    let sex_lower = sex.to_lowercase();
    match sex_lower.as_str() {
        "female" => {
            let covariates = Covariates::female_10_yr_hf();
            let calculation = common_calculation(
                &covariates,
                has_diabetes,
                is_current_smoker,
                on_hypertension_meds,
                _cholesterol_treated,
                systolic_blood_pressure,
                cholesterol_difference,
                hdl_cholesterol,
                age_factor,
                age_squared,
                estimated_glomerular_filtration_rate,
                body_mass_index,
            );
            let risk_score = E.powf(calculation) / (1.0 + E.powf(calculation)) * 100.0;
            Ok(risk_score)
        }
        "male" => {
            let covariates = Covariates::male_10_yr_hf();
            let calculation = common_calculation(
                &covariates,
                has_diabetes,
                is_current_smoker,
                on_hypertension_meds,
                _cholesterol_treated,
                systolic_blood_pressure,
                cholesterol_difference,
                hdl_cholesterol,
                age_factor,
                age_squared,
                estimated_glomerular_filtration_rate,
                body_mass_index,
            );
            let risk_score = E.powf(calculation) / (1.0 + E.powf(calculation)) * 100.0;
            Ok(risk_score)
        }
        _ => Err("Sex must be either 'male' or 'female'.".to_string()),
    }
}

pub fn calculate_30_yr_heart_failure_risk(
    sex: &str,
    age: f64,
    total_cholesterol: f64,
    hdl_cholesterol: f64,
    systolic_bp: f64,
    diabetes: bool,
    smoker: bool,
    bmi: f64,
    egfr: f64,
    on_htn_meds: bool,
    _cholesterol_treated: bool,
) -> Result<f64, String> {
    validate_input(
        age,
        total_cholesterol,
        hdl_cholesterol,
        systolic_bp,
        bmi,
        egfr,
        false,
    )?;

    let cholesterol_diff = total_cholesterol - hdl_cholesterol;
    let age_factor = (age - 55.0) / 10.0;
    let age_factor_squared = age_factor.powi(2);

    match sex.to_lowercase().as_str() {
        "female" => {
            let covariates = Covariates::female_30_yr_hf();
            let calculation = common_calculation(
                &covariates,
                diabetes,
                smoker,
                on_htn_meds,
                _cholesterol_treated,
                systolic_bp,
                cholesterol_diff,
                hdl_cholesterol,
                age_factor,
                age_factor_squared,
                egfr,
                bmi,
            );
            let risk_score = E.powf(calculation) / (1.0 + E.powf(calculation)) * 100.0;
            Ok(risk_score)
        }
        "male" => {
            let covariates = Covariates::male_30_yr_hf();
            let calculation = common_calculation(
                &covariates,
                diabetes,
                smoker,
                on_htn_meds,
                _cholesterol_treated,
                systolic_bp,
                cholesterol_diff,
                hdl_cholesterol,
                age_factor,
                age_factor_squared,
                egfr,
                bmi,
            );
            let risk_score = E.powf(calculation) / (1.0 + E.powf(calculation)) * 100.0;
            Ok(risk_score)
        }
        _ => Err("Sex must be either 'male' or 'female'.".to_string()),
    }
}

#[pyfunction]
pub fn calculate_10_yr_heart_failure_rust(
    sex: String,
    age: f64,
    total_cholesterol: f64,
    hdl_cholesterol: f64,
    systolic_bp: f64,
    has_diabetes: bool,
    is_smoker: bool,
    bmi: f64,
    egfr: f64,
    on_meds: bool,
    _cholesterol_treated: bool,
) -> PyResult<f64> {
    match calculate_10_yr_heart_failure_risk(
        &sex,
        age,
        total_cholesterol,
        hdl_cholesterol,
        systolic_bp,
        has_diabetes,
        is_smoker,
        bmi,
        egfr,
        on_meds,
        _cholesterol_treated,
    ) {
        Ok(value) => Ok(value),
        Err(e) => Err(PyValueError::new_err(e)), // Convert Rust String error to Python ValueError
    }
}

#[pyfunction]
pub fn calculate_30_yr_heart_failure_rust(
    sex: String,
    age: f64,
    total_cholesterol: f64,
    hdl_cholesterol: f64,
    systolic_bp: f64,
    has_diabetes: bool,
    is_smoker: bool,
    bmi: f64,
    egfr: f64,
    on_meds: bool,
    _cholesterol_treated: bool,
) -> PyResult<f64> {
    match calculate_30_yr_heart_failure_risk(
        &sex,
        age,
        total_cholesterol,
        hdl_cholesterol,
        systolic_bp,
        has_diabetes,
        is_smoker,
        bmi,
        egfr,
        on_meds,
        _cholesterol_treated,
    ) {
        Ok(value) => Ok(value),
        Err(e) => Err(PyValueError::new_err(e)), // Convert Rust String error to Python ValueError
    }
}

#[pyfunction]
pub fn calculate_10_yr_hf_rust_parallel_np(
    py: Python,
    data: PyReadonlyArrayDyn<f64>,
) -> PyResult<PyObject> {
    calculate_risk_rust_parallel_np(py, data, calculate_10_yr_heart_failure_risk)
}

#[pyfunction]
pub fn calculate_30_yr_hf_rust_parallel_np(
    py: Python,
    data: PyReadonlyArrayDyn<f64>,
) -> PyResult<PyObject> {
    calculate_risk_rust_parallel_np(py, data, calculate_30_yr_heart_failure_risk)
}
