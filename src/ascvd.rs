use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use std::f64;
use std::f64::consts::E;
use rayon::prelude::*;
use numpy::{PyArray, PyReadonlyArrayDyn};


pub fn calculate_10_yr_ascvd_risk(
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

    // Input validation (adjust according to your specific requirements)
  if !(30.0..=79.0).contains(&age) {
        return Err("Age must be between 30 and 79".to_string());
    }
    if !(130.0..=320.0).contains(&total_cholesterol) {
        return Err("Total cholesterol must be between 130 and 320".to_string());
    }
    if !(20.0..=100.0).contains(&hdl_cholesterol) {
        return Err("HDL cholesterol must be between 20 and 100".to_string());
    }
    if !(90.0..=200.0).contains(&systolic_bp) {
        return Err("Systolic blood pressure must be between 90 and 200".to_string());
    }
    if !(18.5..=39.9).contains(&bmi) {
        return Err("BMI must be between 18.5 and 39.9".to_string());
    }
    if !(15.0..=140.0).contains(&egfr) {
        return Err("eGFR must be between 15 and 140".to_string());
    }

    let cholesterol_diff = total_cholesterol - hdl_cholesterol;
    let age_adjusted = (age - 55.0) / 10.0;

    match sex.to_lowercase().as_str() {
        "female" => {
            let diabetes_factor = if has_diabetes { 0.8348585 } else { 0.0 };
            let smoker_factor = if current_smoker { 0.4831078 } else { 0.0 };
            let htn_meds_factor = if on_htn_meds { 0.2265309 } else { 0.0 };
            let htn_cholesterol_treatment_factor = if on_cholesterol_meds { 0.0592374 } else { 0.0 };
            let systolic_bp_adjusted_max = (systolic_bp.max(110.0) - 130.0) / 20.0;
            let cholesterol_diff_factor = 0.02586 * cholesterol_diff - 3.5;

            let diabetes_age_factor = if has_diabetes { 0.2417542 * age_adjusted } else { 0.0 };
            let smoker_age_factor = if current_smoker { 0.0791142 * age_adjusted } else { 0.0 };

            let calculation = 0.719883 * age_adjusted
                - 3.819975
                + 0.1176967 * cholesterol_diff_factor
                - 0.151185 * (0.02586 * hdl_cholesterol - 1.3) / 0.3
                - 0.0835358 * (systolic_bp.min(110.0) - 110.0) / 20.0
                + 0.3592852 * systolic_bp_adjusted_max
                + diabetes_factor
                + smoker_factor
                + 0.4864619 * (egfr.min(60.0) - 60.0) / -15.0
                + 0.0397779 * (egfr.max(60.0) - 90.0) / -15.0
                + htn_meds_factor
                - htn_cholesterol_treatment_factor
                - (if on_htn_meds { 0.0395762 * systolic_bp_adjusted_max } else { 0.0 })
                + (if on_cholesterol_meds { 0.0844423 * cholesterol_diff_factor } else { 0.0 })
                - 0.0567839 * age_adjusted * cholesterol_diff_factor
                + 0.0325692 * age_adjusted * (0.02586 * hdl_cholesterol - 1.3) / 0.3
                - 0.1035985 * age_adjusted * systolic_bp_adjusted_max
                - diabetes_age_factor
                - smoker_age_factor
                - 0.1671492 * age_adjusted * (egfr.min(60.0) - 60.0) / -15.0;

            let risk_score = E.powf(calculation) / (1.0 + E.powf(calculation)) * 100.0;
            Ok(risk_score)
        }
        "male" => {
            let diabetes_factor = if has_diabetes { 0.7189597 } else { 0.0 };
            let smoker_factor = if current_smoker { 0.3956973 } else { 0.0 };
            let htn_meds_factor = if on_htn_meds { 0.2036522 } else { 0.0 };
            let htn_cholesterol_treatment_factor = if on_cholesterol_meds { 0.0865581 } else { 0.0 };
            let systolic_bp_adjusted_max = (systolic_bp.max(110.0) - 130.0) / 20.0;
            let cholesterol_diff_factor = 0.02586 * cholesterol_diff - 3.5;

            let diabetes_age_factor = if has_diabetes { 0.2018525 * age_adjusted } else { 0.0 };
            let smoker_age_factor = if current_smoker { 0.0970527 * age_adjusted } else { 0.0 };

            let calculation = 0.7099847 * age_adjusted
                - 3.500655
                + 0.1658663 * cholesterol_diff_factor
                - 0.1144285 * (0.02586 * hdl_cholesterol - 1.3) / 0.3
                - 0.2837212 * (systolic_bp.min(110.0) - 110.0) / 20.0
                + 0.3239977 * systolic_bp_adjusted_max
                + diabetes_factor
                + smoker_factor
                + 0.3690075 * (egfr.min(60.0) - 60.0) / -15.0
                + 0.0203619 * (egfr.max(60.0) - 90.0) / -15.0
                + htn_meds_factor
                - htn_cholesterol_treatment_factor
                - (if on_htn_meds { 0.0322916 * systolic_bp_adjusted_max } else { 0.0 })
                + (if on_cholesterol_meds { 0.114563 * cholesterol_diff_factor } else { 0.0 })
                - 0.0300005 * age_adjusted * cholesterol_diff_factor
                + 0.0232747 * age_adjusted * (0.02586 * hdl_cholesterol - 1.3) / 0.3
                - 0.0927024 * age_adjusted * systolic_bp_adjusted_max
                - diabetes_age_factor
                - smoker_age_factor
                - 0.1217081 * age_adjusted * (egfr.min(60.0) - 60.0) / -15.0;

            let risk_score = E.powf(calculation) / (1.0 + E.powf(calculation)) * 100.0;
            Ok(risk_score)
        }
        _ => Err("Sex must be either 'male' or 'female'.".to_string()),
    }
}



pub fn calculate_30_yr_ascvd_value(
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
    cholesterol_treated: bool,
) -> Result<f64, String> {

    // Add input validation here as needed...
    // Example validation:
  if !(30.0..=59.0).contains(&age) {
        return Err("Age must be between 30 and 59".to_string());
    }
    if !(130.0..=320.0).contains(&total_cholesterol) {
        return Err("Total cholesterol must be between 130 and 320".to_string());
    }
    if !(20.0..=100.0).contains(&hdl_cholesterol) {
        return Err("HDL cholesterol must be between 20 and 100".to_string());
    }
    if !(90.0..=200.0).contains(&systolic_bp) {
        return Err("Systolic blood pressure must be between 90 and 200".to_string());
    }
    if !(18.5..=39.9).contains(&bmi) {
        return Err("BMI must be between 18.5 and 39.9".to_string());
    }
    if !(15.0..=140.0).contains(&egfr) {
        return Err("eGFR must be between 15 and 140".to_string());
    }

    let cholesterol_difference = total_cholesterol - hdl_cholesterol;
    let age_factor = (age - 55.0) / 10.0;
    let age_squared = age_factor.powi(2);

    match sex.to_lowercase().as_str() {
        "female" => {
            let calculation = 0.4669202 * age_factor
                - 1.974074
                - 0.0893118 * age_squared
                + 0.1256901 * (0.02586 * cholesterol_difference - 3.5)
                - 0.1542255 * (0.02586 * hdl_cholesterol - 1.3) / 0.3
                - 0.0018093 * (systolic_bp.min(110.0) - 110.0) / 20.0
                + 0.322949 * (systolic_bp.max(110.0) - 130.0) / 20.0
                + if has_diabetes { 0.6296707 } else { 0.0 }
                + if current_smoker { 0.268292 } else { 0.0 }
                + 0.100106 * (egfr.min(60.0) - 60.0) / -15.0
                + 0.0499663 * (egfr.max(60.0) - 90.0) / -15.0
                + if on_htn_meds { 0.1875292 } else { 0.0 }
                + if cholesterol_treated { 0.0152476 } else { 0.0 }
                - if on_htn_meds { 0.0276123 * (systolic_bp.max(110.0) - 130.0) / 20.0 } else { 0.0 }
                + if cholesterol_treated { 0.0736147 * (0.02586 * cholesterol_difference - 3.5) } else { 0.0 }
                - 0.0521962 * age_factor * (0.02586 * cholesterol_difference - 3.5)
                + 0.0316918 * age_factor * (0.02586 * hdl_cholesterol - 1.3) / 0.3
                - 0.1046101 * age_factor * (systolic_bp.max(110.0) - 130.0) / 20.0
                - if has_diabetes { 0.2727793 * age_factor } else { 0.0 }
                - if current_smoker { 0.1530907 * age_factor } else { 0.0 }
                - 0.1299149 * age_factor * (egfr.min(60.0) - 60.0) / -15.0;

            let risk_score = E.powf(calculation) / (1.0 + E.powf(calculation)) * 100.0;
            Ok(risk_score)
        }
        "male" => {
            let calculation = 0.3994099 * age_factor
                - 1.736444
                - 0.0937484 * age_squared
                + 0.1744643 * (0.02586 * cholesterol_difference - 3.5)
                - 0.120203 * (0.02586 * hdl_cholesterol - 1.3) / 0.3
                - 0.0665117 * (systolic_bp.min(110.0) - 110.0) / 20.0
                + 0.2753037 * (systolic_bp.max(110.0) - 130.0) / 20.0
                + if has_diabetes { 0.4790257 } else { 0.0 }
                + if current_smoker { 0.1782635 } else { 0.0 }
                - 0.0218789 * (egfr.min(60.0) - 60.0) / -15.0
                + 0.0602553 * (egfr.max(60.0) - 90.0) / -15.0
                + if on_htn_meds { 0.1421182 } else { 0.0 }
                + if cholesterol_treated { 0.0135996 } else { 0.0 }
                - if on_htn_meds { 0.0218265 * (systolic_bp.max(110.0) - 130.0) / 20.0 } else { 0.0 }
                + if cholesterol_treated { 0.1013148 * (0.02586 * cholesterol_difference - 3.5) } else { 0.0 }
                - 0.0312619 * age_factor * (0.02586 * cholesterol_difference - 3.5)
                + 0.020673 * age_factor * (0.02586 * hdl_cholesterol - 1.3) / 0.3
                - 0.0920935 * age_factor * (systolic_bp.max(110.0) - 130.0) / 20.0
                - if has_diabetes { 0.2159947 * age_factor } else { 0.0 }
                - if current_smoker { 0.1548811 * age_factor } else { 0.0 }
                - 0.0712547 * age_factor * (egfr.min(60.0) - 60.0) / -15.0;

            let risk_score = E.powf(calculation) / (1.0 + E.powf(calculation)) * 100.0;
            Ok(risk_score)
        }
        _ => Err("Sex must be either 'male' or 'female'.".to_string()),
    }
}


#[pyfunction]
pub fn calculate_10_yr_ascvd_rust(sex: String, age: f64, total_cholesterol: f64, hdl_cholesterol: f64,
                                    systolic_bp: f64, has_diabetes: bool, current_smoker: bool, bmi: f64,
                                    egfr: f64, on_htn_meds: bool, on_cholesterol_meds: bool) -> PyResult<f64> {
    match calculate_10_yr_ascvd_risk(&sex, age, total_cholesterol, hdl_cholesterol, systolic_bp, has_diabetes, current_smoker, bmi, egfr, on_htn_meds, on_cholesterol_meds) {
        Ok(value) => Ok(value),
        Err(e) => Err(PyValueError::new_err(e)), // Convert Rust String error to Python ValueError
    }
}


#[pyfunction]
pub fn calculate_30_yr_ascvd_rust(sex: String, age: f64, total_cholesterol: f64, hdl_cholesterol: f64,
                                    systolic_bp: f64, has_diabetes: bool, is_smoker: bool, bmi: f64,
                                    egfr: f64, on_meds: bool, cholesterol_treated: bool) -> PyResult<f64> {
    match calculate_30_yr_ascvd_value(&sex, age, total_cholesterol, hdl_cholesterol, systolic_bp, has_diabetes, is_smoker, bmi, egfr, on_meds, cholesterol_treated) {
        Ok(value) => Ok(value),
        Err(e) => Err(PyValueError::new_err(e)), // Convert Rust String error to Python ValueError
    }
}

#[pyfunction]
pub fn calculate_10_yr_ascvd_rust_parallel(data: Vec<(String, f64, f64, f64, f64, bool, bool, f64, f64, bool, bool)>) -> PyResult<Vec<f64>> {
    let results: Vec<_> = data.par_iter()
        .map(|(sex, age, total_cholesterol, hdl_cholesterol, systolic_bp, has_diabetes, current_smoker, bmi, egfr, on_htn_meds, on_cholesterol_meds)| {
            calculate_10_yr_ascvd_risk(sex, *age, *total_cholesterol, *hdl_cholesterol, *systolic_bp, *has_diabetes, *current_smoker, *bmi, *egfr, *on_htn_meds, *on_cholesterol_meds)
        })
        .collect();

    results.into_iter()
           .map(|res| res.map_err(|e| PyValueError::new_err(e)))
           .collect()
}

#[pyfunction]
pub fn calculate_10_yr_ascvd_rust_parallel_np(
    py: Python,
    data: PyReadonlyArrayDyn<f64>
) -> PyResult<PyObject> {
    let shape = data.shape();
    if shape.len() != 2 || shape[1] != 11 {
        return Err(PyValueError::new_err("Array shape must be (n, 11)"));
    }

    let rows = data.as_array().outer_iter()
        .map(|row| (
            if row[0] == 1.0 { "male" } else { "female" }, // Convert numeric to "male" or "female"
            row[1], row[2], row[3], row[4],
            row[5] != 0.0, // Convert float to bool
            row[6] != 0.0, // Convert float to bool
            row[7], row[8],
            row[9] != 0.0, // Convert float to bool
            row[10] != 0.0 // Convert float to bool
        ))
        .collect::<Vec<_>>();

    let results: Vec<_> = rows.into_par_iter()
        .map(|(sex, age, total_cholesterol, hdl_cholesterol, systolic_bp, has_diabetes, current_smoker, bmi, egfr, on_htn_meds, on_cholesterol_meds)| {
            calculate_10_yr_ascvd_risk(sex, age, total_cholesterol, hdl_cholesterol, systolic_bp, has_diabetes, current_smoker, bmi, egfr, on_htn_meds, on_cholesterol_meds)
                .unwrap_or(f64::NAN) // Handle error by returning NaN
        })
        .collect();

    Ok(PyArray::from_vec(py, results).to_object(py))
}