use crate::covariates::Covariates;
use crate::utils::{common_calculation, validate_input};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use std::f64;
use std::f64::consts::E;

pub fn calculate_10_yr_cvd_risk(
    sex: &str,
    age: f64,
    total_cholesterol: f64,
    hdl_cholesterol: f64,
    systolic_bp: f64,
    has_diabetes: bool,
    current_smoker: bool,
    bmi: f64,
    egfr: f64,
    on_htn_meds: bool,
    on_cholesterol_meds: bool,
) -> Result<f64, String> {
    validate_input(
        age,
        total_cholesterol,
        hdl_cholesterol,
        systolic_bp,
        bmi,
        egfr,
        true,
    )?;

    let cholesterol_diff = total_cholesterol - hdl_cholesterol;
    let adjusted_age = (age - 55.0) / 10.0;
    let adjusted_age_squared = adjusted_age.powi(2);

    match sex.to_lowercase().as_str() {
        "female" => {
            let covariates = Covariates::female_10_yr_cvd();
            let calculation = common_calculation(
                &covariates,
                has_diabetes,
                current_smoker,
                on_htn_meds,
                on_cholesterol_meds,
                systolic_bp,
                cholesterol_diff,
                hdl_cholesterol,
                adjusted_age,
                adjusted_age_squared,
                egfr,
            );
            let risk_score = E.powf(calculation) / (1.0 + E.powf(calculation)) * 100.0;
            Ok(risk_score)
        }
        "male" => {
            let covariates = Covariates::male_10_yr_cvd();
            let calculation = common_calculation(
                &covariates,
                has_diabetes,
                current_smoker,
                on_htn_meds,
                on_cholesterol_meds,
                systolic_bp,
                cholesterol_diff,
                hdl_cholesterol,
                adjusted_age,
                adjusted_age_squared,
                egfr,
            );
            let risk_score = E.powf(calculation) / (1.0 + E.powf(calculation)) * 100.0;
            Ok(risk_score)
        }
        _ => Err("Sex must be either 'male' or 'female'.".to_string()),
    }
}

pub fn calculate_30_yr_cvd_risk(
    sex: &str,
    age: f64,
    total_cholesterol: f64,
    hdl_cholesterol: f64,
    systolic_bp: f64,
    has_diabetes: bool,
    current_smoker: bool,
    bmi: f64,
    egfr: f64,
    on_htn_meds: bool,
    on_cholesterol_meds: bool,
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

    let cholesterol_difference = total_cholesterol - hdl_cholesterol;
    let age_factor = (age - 55.0) / 10.0;
    let age_squared = age_factor.powi(2);

    match sex.to_lowercase().as_str() {
        "female" => {
            let covariates = Covariates::female_30_yr_cvd();
            let calculation = common_calculation(
                &covariates,
                has_diabetes,
                current_smoker,
                on_htn_meds,
                on_cholesterol_meds,
                systolic_bp,
                cholesterol_difference,
                hdl_cholesterol,
                age_factor,
                age_squared,
                egfr,
            );
            let risk_score = E.powf(calculation) / (1.0 + E.powf(calculation)) * 100.0;
            Ok(risk_score)
        }
        "male" => {
            let calculation = 0.4627309 * age_factor - 1.148204 - 0.0984281 * age_squared
                + 0.0836088 * (0.02586 * cholesterol_difference - 3.5)
                - 0.1029824 * (0.02586 * hdl_cholesterol - 1.3) / 0.3
                - 0.2140352 * (systolic_bp.min(110.0) - 110.0) / 20.0
                + 0.2904325 * (systolic_bp.max(110.0) - 130.0) / 20.0
                + if has_diabetes { 0.5331276 } else { 0.0 }
                + if current_smoker { 0.2141914 } else { 0.0 }
                + 0.1155556 * (egfr.min(60.0) - 60.0) / -15.0
                + 0.0603775 * (egfr.max(60.0) - 90.0) / -15.0
                + if on_htn_meds { 0.232714 } else { 0.0 }
                - if on_cholesterol_meds { 0.0272112 } else { 0.0 }
                - if on_htn_meds {
                    0.0384488 * (systolic_bp.max(110.0) - 130.0) / 20.0
                } else {
                    0.0
                }
                + if on_cholesterol_meds {
                    0.134192 * (0.02586 * cholesterol_difference - 3.5)
                } else {
                    0.0
                }
                - 0.0511759 * age_factor * (0.02586 * cholesterol_difference - 3.5)
                + 0.0165865 * age_factor * (0.02586 * hdl_cholesterol - 1.3) / 0.3
                - 0.1101437 * age_factor * (systolic_bp.max(110.0) - 130.0) / 20.0
                - if has_diabetes {
                    0.2585943 * age_factor
                } else {
                    0.0
                }
                - if current_smoker {
                    0.1566406 * age_factor
                } else {
                    0.0
                }
                - 0.1166776 * age_factor * (egfr.min(60.0) - 60.0) / -15.0;

            let risk_score = E.powf(calculation) / (1.0 + E.powf(calculation)) * 100.0;
            Ok(risk_score)
        }
        _ => Err("Sex must be either 'male' or 'female'.".to_string()),
    }
}

#[pyfunction]
pub fn calculate_10_yr_cvd_rust(
    sex: String,
    age: f64,
    total_cholesterol: f64,
    hdl_cholesterol: f64,
    systolic_bp: f64,
    has_diabetes: bool,
    current_smoker: bool,
    bmi: f64,
    egfr: f64,
    on_htn_meds: bool,
    on_cholesterol_meds: bool,
) -> PyResult<f64> {
    match calculate_10_yr_cvd_risk(
        &sex,
        age,
        total_cholesterol,
        hdl_cholesterol,
        systolic_bp,
        has_diabetes,
        current_smoker,
        bmi,
        egfr,
        on_htn_meds,
        on_cholesterol_meds,
    ) {
        Ok(value) => Ok(value),
        Err(e) => Err(PyValueError::new_err(e)), // Convert Rust String error to Python ValueError
    }
}

#[pyfunction]
pub fn calculate_30_yr_cvd_rust(
    sex: String,
    age: f64,
    total_cholesterol: f64,
    hdl_cholesterol: f64,
    systolic_bp: f64,
    has_diabetes: bool,
    current_smoker: bool,
    bmi: f64,
    egfr: f64,
    on_htn_meds: bool,
    on_cholesterol_meds: bool,
) -> PyResult<f64> {
    match calculate_30_yr_cvd_risk(
        &sex,
        age,
        total_cholesterol,
        hdl_cholesterol,
        systolic_bp,
        has_diabetes,
        current_smoker,
        bmi,
        egfr,
        on_htn_meds,
        on_cholesterol_meds,
    ) {
        Ok(value) => Ok(value),
        Err(e) => Err(PyValueError::new_err(e)), // Convert Rust String error to Python ValueError
    }
}
