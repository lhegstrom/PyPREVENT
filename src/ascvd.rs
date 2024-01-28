use crate::multipliers::Multipliers;
use crate::utils::validate_input;
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
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
    let age_adjusted = (age - 55.0) / 10.0;

    match sex.to_lowercase().as_str() {
        "female" => {
            let female_multipliers = Multipliers::female_10_yr_ascvd();

            let diabetes_factor = if has_diabetes {
                female_multipliers.diabetes_factor
            } else {
                0.0
            };
            let smoker_factor = if current_smoker {
                female_multipliers.smoker_factor
            } else {
                0.0
            };
            let htn_meds_factor = if on_htn_meds {
                female_multipliers.htn_meds_factor
            } else {
                0.0
            };
            let htn_cholesterol_treatment_factor = if on_cholesterol_meds {
                female_multipliers.cholesterol_meds_factor
            } else {
                0.0
            };
            let systolic_bp_adjusted_max = (systolic_bp.max(110.0) - 130.0) / 20.0;
            let cholesterol_diff_factor =
                female_multipliers.cholesterol_base_multiplier * cholesterol_diff - 3.5;

            let diabetes_age_factor = if has_diabetes {
                female_multipliers.diabetes_age_factor * age_adjusted
            } else {
                0.0
            };
            let smoker_age_factor = if current_smoker {
                female_multipliers.smoker_age_factor * age_adjusted
            } else {
                0.0
            };

            let calculation = vec![
                female_multipliers.age_adjustment_factor * age_adjusted,
                female_multipliers.constant,
                female_multipliers.total_cholesterol_diff_factor * cholesterol_diff_factor,
                female_multipliers.hdl_cholesterol_diff_factor
                    * (female_multipliers.cholesterol_base_multiplier * hdl_cholesterol - 1.3)
                    / 0.3,
                female_multipliers.systolic_bp_min_factor * (systolic_bp.min(110.0) - 110.0) / 20.0,
                female_multipliers.systolic_bp_max_factor * systolic_bp_adjusted_max,
                diabetes_factor,
                smoker_factor,
                female_multipliers.egfr_min_factor * (egfr.min(60.0) - 60.0) / -15.0,
                female_multipliers.egfr_max_factor * (egfr.max(60.0) - 90.0) / -15.0,
                htn_meds_factor,
                htn_cholesterol_treatment_factor,
                (if on_htn_meds {
                    female_multipliers.htn_meds_systolic_bp_max_factor * systolic_bp_adjusted_max
                } else {
                    0.0
                }),
                (if on_cholesterol_meds {
                    female_multipliers.cholesterol_meds_cholesterol_diff_factor
                        * cholesterol_diff_factor
                } else {
                    0.0
                }),
                female_multipliers.age_adjustment_cholesterol_diff_factor
                    * age_adjusted
                    * cholesterol_diff_factor,
                female_multipliers.age_adjustment_hdl_cholesterol_diff_factor
                    * age_adjusted
                    * (female_multipliers.cholesterol_base_multiplier * hdl_cholesterol - 1.3)
                    / 0.3,
                female_multipliers.age_adjustment_systolic_bp_max_factor
                    * age_adjusted
                    * systolic_bp_adjusted_max,
                diabetes_age_factor,
                smoker_age_factor,
                female_multipliers.age_min_egfr_factor * age_adjusted * (egfr.min(60.0) - 60.0)
                    / -15.0,
            ]
            .iter()
            .sum();

            let risk_score = E.powf(calculation) / (1.0 + E.powf(calculation)) * 100.0;
            Ok(risk_score)
        }
        "male" => {
            let male_multipliers = Multipliers::male_10_yr_ascvd();

            let diabetes_factor = if has_diabetes {
                male_multipliers.diabetes_factor
            } else {
                0.0
            };
            let smoker_factor = if current_smoker {
                male_multipliers.smoker_factor
            } else {
                0.0
            };
            let htn_meds_factor = if on_htn_meds {
                male_multipliers.htn_meds_factor
            } else {
                0.0
            };
            let htn_cholesterol_treatment_factor = if on_cholesterol_meds {
                male_multipliers.cholesterol_meds_factor
            } else {
                0.0
            };
            let systolic_bp_adjusted_max = (systolic_bp.max(110.0) - 130.0) / 20.0;
            let cholesterol_diff_factor =
                male_multipliers.cholesterol_base_multiplier * cholesterol_diff - 3.5;

            let diabetes_age_factor = if has_diabetes {
                male_multipliers.diabetes_age_factor * age_adjusted
            } else {
                0.0
            };
            let smoker_age_factor = if current_smoker {
                male_multipliers.smoker_age_factor * age_adjusted
            } else {
                0.0
            };

            let calculation = vec![
                male_multipliers.age_adjustment_factor * age_adjusted,
                male_multipliers.constant,
                male_multipliers.total_cholesterol_diff_factor * cholesterol_diff_factor,
                male_multipliers.hdl_cholesterol_diff_factor
                    * (male_multipliers.cholesterol_base_multiplier * hdl_cholesterol - 1.3)
                    / 0.3,
                male_multipliers.systolic_bp_min_factor * (systolic_bp.min(110.0) - 110.0) / 20.0,
                male_multipliers.systolic_bp_max_factor * systolic_bp_adjusted_max,
                diabetes_factor,
                smoker_factor,
                male_multipliers.egfr_min_factor * (egfr.min(60.0) - 60.0) / -15.0,
                male_multipliers.egfr_max_factor * (egfr.max(60.0) - 90.0) / -15.0,
                htn_meds_factor,
                htn_cholesterol_treatment_factor,
                (if on_htn_meds {
                    male_multipliers.htn_meds_systolic_bp_max_factor * systolic_bp_adjusted_max
                } else {
                    0.0
                }),
                (if on_cholesterol_meds {
                    male_multipliers.cholesterol_meds_cholesterol_diff_factor
                        * cholesterol_diff_factor
                } else {
                    0.0
                }),
                male_multipliers.age_adjustment_cholesterol_diff_factor
                    * age_adjusted
                    * cholesterol_diff_factor,
                male_multipliers.age_adjustment_hdl_cholesterol_diff_factor
                    * age_adjusted
                    * (male_multipliers.cholesterol_base_multiplier * hdl_cholesterol - 1.3)
                    / 0.3,
                male_multipliers.age_adjustment_systolic_bp_max_factor
                    * age_adjusted
                    * systolic_bp_adjusted_max,
                diabetes_age_factor,
                smoker_age_factor,
                male_multipliers.age_min_egfr_factor * age_adjusted * (egfr.min(60.0) - 60.0)
                    / -15.0,
            ]
            .iter()
            .sum();

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
            let calculation = 0.4669202 * age_factor - 1.974074 - 0.0893118 * age_squared
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
                - if on_htn_meds {
                    0.0276123 * (systolic_bp.max(110.0) - 130.0) / 20.0
                } else {
                    0.0
                }
                + if cholesterol_treated {
                    0.0736147 * (0.02586 * cholesterol_difference - 3.5)
                } else {
                    0.0
                }
                - 0.0521962 * age_factor * (0.02586 * cholesterol_difference - 3.5)
                + 0.0316918 * age_factor * (0.02586 * hdl_cholesterol - 1.3) / 0.3
                - 0.1046101 * age_factor * (systolic_bp.max(110.0) - 130.0) / 20.0
                - if has_diabetes {
                    0.2727793 * age_factor
                } else {
                    0.0
                }
                - if current_smoker {
                    0.1530907 * age_factor
                } else {
                    0.0
                }
                - 0.1299149 * age_factor * (egfr.min(60.0) - 60.0) / -15.0;

            let risk_score = E.powf(calculation) / (1.0 + E.powf(calculation)) * 100.0;
            Ok(risk_score)
        }
        "male" => {
            let calculation = 0.3994099 * age_factor - 1.736444 - 0.0937484 * age_squared
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
                - if on_htn_meds {
                    0.0218265 * (systolic_bp.max(110.0) - 130.0) / 20.0
                } else {
                    0.0
                }
                + if cholesterol_treated {
                    0.1013148 * (0.02586 * cholesterol_difference - 3.5)
                } else {
                    0.0
                }
                - 0.0312619 * age_factor * (0.02586 * cholesterol_difference - 3.5)
                + 0.020673 * age_factor * (0.02586 * hdl_cholesterol - 1.3) / 0.3
                - 0.0920935 * age_factor * (systolic_bp.max(110.0) - 130.0) / 20.0
                - if has_diabetes {
                    0.2159947 * age_factor
                } else {
                    0.0
                }
                - if current_smoker {
                    0.1548811 * age_factor
                } else {
                    0.0
                }
                - 0.0712547 * age_factor * (egfr.min(60.0) - 60.0) / -15.0;

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
