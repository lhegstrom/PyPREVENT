use crate::covariates::Covariates;
use crate::utils::{common_calculation, validate_input};
use numpy::{PyArray, PyReadonlyArrayDyn};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use rayon::prelude::*;
use std::f64;
use std::f64::consts::E;

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
            let covariates = Covariates::female_10_yr_ascvd();
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
                bmi,
            );
            let risk_score = E.powf(calculation) / (1.0 + E.powf(calculation)) * 100.0;
            Ok(risk_score)
        }
        "male" => {
            let covariates = Covariates::male_10_yr_ascvd();
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
                bmi,
            );
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

    let cholesterol_diff = total_cholesterol - hdl_cholesterol;
    let adjusted_age = (age - 55.0) / 10.0;
    let adjusted_age_squared = adjusted_age.powi(2);

    match sex.to_lowercase().as_str() {
        "female" => {
            let covariates = Covariates::female_30_yr_ascvd();
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
                bmi,
            );
            let risk_score = E.powf(calculation) / (1.0 + E.powf(calculation)) * 100.0;
            Ok(risk_score)
        }
        "male" => {
            let covariates = Covariates::male_30_yr_ascvd();
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
                bmi,
            );
            let risk_score = E.powf(calculation) / (1.0 + E.powf(calculation)) * 100.0;
            Ok(risk_score)
        }
        _ => Err("Sex must be either 'male' or 'female'.".to_string()),
    }
}

#[pyfunction]
pub fn calculate_10_yr_ascvd_rust(
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
    match calculate_10_yr_ascvd_risk(
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
pub fn calculate_30_yr_ascvd_rust(
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
    cholesterol_treated: bool,
) -> PyResult<f64> {
    match calculate_30_yr_ascvd_value(
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
        cholesterol_treated,
    ) {
        Ok(value) => Ok(value),
        Err(e) => Err(PyValueError::new_err(e)), // Convert Rust String error to Python ValueError
    }
}

#[pyfunction]
pub fn calculate_10_yr_ascvd_rust_parallel(
    data: Vec<(String, f64, f64, f64, f64, bool, bool, f64, f64, bool, bool)>,
) -> PyResult<Vec<f64>> {
    let results: Vec<_> = data
        .par_iter()
        .map(
            |(
                sex,
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
            )| {
                calculate_10_yr_ascvd_risk(
                    sex,
                    *age,
                    *total_cholesterol,
                    *hdl_cholesterol,
                    *systolic_bp,
                    *has_diabetes,
                    *current_smoker,
                    *bmi,
                    *egfr,
                    *on_htn_meds,
                    *on_cholesterol_meds,
                )
            },
        )
        .collect();

    results
        .into_iter()
        .map(|res| res.map_err(|e| PyValueError::new_err(e)))
        .collect()
}

#[pyfunction]
pub fn calculate_10_yr_ascvd_rust_parallel_np(
    py: Python,
    data: PyReadonlyArrayDyn<f64>,
) -> PyResult<PyObject> {
    let shape = data.shape();
    if shape.len() != 2 || shape[1] != 11 {
        return Err(PyValueError::new_err("Array shape must be (n, 11)"));
    }

    let rows = data
        .as_array()
        .outer_iter()
        .map(|row| {
            (
                if row[0] == 1.0 { "male" } else { "female" }, // Convert numeric to "male" or "female"
                row[1],
                row[2],
                row[3],
                row[4],
                row[5] != 0.0, // Convert float to bool
                row[6] != 0.0, // Convert float to bool
                row[7],
                row[8],
                row[9] != 0.0,  // Convert float to bool
                row[10] != 0.0, // Convert float to bool
            )
        })
        .collect::<Vec<_>>();

    let results: Vec<_> = rows
        .into_par_iter()
        .map(
            |(
                sex,
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
            )| {
                calculate_10_yr_ascvd_risk(
                    sex,
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
                )
                .unwrap_or(f64::NAN) // Handle error by returning NaN
            },
        )
        .collect();

    Ok(PyArray::from_vec(py, results).to_object(py))
}
