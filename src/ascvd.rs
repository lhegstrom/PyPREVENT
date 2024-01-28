use crate::covariates::Covariates;
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
            let covariates = Covariates::female_10_yr_ascvd();

            let diabetes_factor = if has_diabetes {
                covariates.diabetes_factor
            } else {
                0.0
            };
            let smoker_factor = if current_smoker {
                covariates.smoker_factor
            } else {
                0.0
            };
            let htn_meds_factor = if on_htn_meds {
                covariates.htn_meds_factor
            } else {
                0.0
            };
            let htn_cholesterol_treatment_factor = if on_cholesterol_meds {
                covariates.cholesterol_meds_factor
            } else {
                0.0
            };
            let systolic_bp_adjusted_max = (systolic_bp.max(110.0) - 130.0) / 20.0;
            let cholesterol_diff_factor =
                covariates.cholesterol_base_multiplier * cholesterol_diff - 3.5;

            let diabetes_age_factor = if has_diabetes {
                covariates.diabetes_age_factor * age_adjusted
            } else {
                0.0
            };
            let smoker_age_factor = if current_smoker {
                covariates.smoker_age_factor * age_adjusted
            } else {
                0.0
            };

            let calculation = vec![
                covariates.age_adjustment_factor * age_adjusted,
                covariates.constant,
                covariates.total_cholesterol_diff_factor * cholesterol_diff_factor,
                covariates.hdl_cholesterol_diff_factor
                    * (covariates.cholesterol_base_multiplier * hdl_cholesterol - 1.3)
                    / 0.3,
                covariates.systolic_bp_min_factor * (systolic_bp.min(110.0) - 110.0) / 20.0,
                covariates.systolic_bp_max_factor * systolic_bp_adjusted_max,
                diabetes_factor,
                smoker_factor,
                covariates.egfr_min_factor * (egfr.min(60.0) - 60.0) / -15.0,
                covariates.egfr_max_factor * (egfr.max(60.0) - 90.0) / -15.0,
                htn_meds_factor,
                htn_cholesterol_treatment_factor,
                (if on_htn_meds {
                    covariates.htn_meds_systolic_bp_max_factor * systolic_bp_adjusted_max
                } else {
                    0.0
                }),
                (if on_cholesterol_meds {
                    covariates.cholesterol_meds_cholesterol_diff_factor * cholesterol_diff_factor
                } else {
                    0.0
                }),
                covariates.age_adjustment_cholesterol_diff_factor
                    * age_adjusted
                    * cholesterol_diff_factor,
                covariates.age_adjustment_hdl_cholesterol_diff_factor
                    * age_adjusted
                    * (covariates.cholesterol_base_multiplier * hdl_cholesterol - 1.3)
                    / 0.3,
                covariates.age_adjustment_systolic_bp_max_factor
                    * age_adjusted
                    * systolic_bp_adjusted_max,
                diabetes_age_factor,
                smoker_age_factor,
                covariates.age_min_egfr_factor * age_adjusted * (egfr.min(60.0) - 60.0) / -15.0,
            ]
            .iter()
            .sum();

            let risk_score = E.powf(calculation) / (1.0 + E.powf(calculation)) * 100.0;
            Ok(risk_score)
        }
        "male" => {
            let covariates = Covariates::male_10_yr_ascvd();

            let diabetes_factor = if has_diabetes {
                covariates.diabetes_factor
            } else {
                0.0
            };
            let smoker_factor = if current_smoker {
                covariates.smoker_factor
            } else {
                0.0
            };
            let htn_meds_factor = if on_htn_meds {
                covariates.htn_meds_factor
            } else {
                0.0
            };
            let htn_cholesterol_treatment_factor = if on_cholesterol_meds {
                covariates.cholesterol_meds_factor
            } else {
                0.0
            };
            let systolic_bp_adjusted_max = (systolic_bp.max(110.0) - 130.0) / 20.0;
            let cholesterol_diff_factor =
                covariates.cholesterol_base_multiplier * cholesterol_diff - 3.5;

            let diabetes_age_factor = if has_diabetes {
                covariates.diabetes_age_factor * age_adjusted
            } else {
                0.0
            };
            let smoker_age_factor = if current_smoker {
                covariates.smoker_age_factor * age_adjusted
            } else {
                0.0
            };

            let calculation = vec![
                covariates.age_adjustment_factor * age_adjusted,
                covariates.constant,
                covariates.total_cholesterol_diff_factor * cholesterol_diff_factor,
                covariates.hdl_cholesterol_diff_factor
                    * (covariates.cholesterol_base_multiplier * hdl_cholesterol - 1.3)
                    / 0.3,
                covariates.systolic_bp_min_factor * (systolic_bp.min(110.0) - 110.0) / 20.0,
                covariates.systolic_bp_max_factor * systolic_bp_adjusted_max,
                diabetes_factor,
                smoker_factor,
                covariates.egfr_min_factor * (egfr.min(60.0) - 60.0) / -15.0,
                covariates.egfr_max_factor * (egfr.max(60.0) - 90.0) / -15.0,
                htn_meds_factor,
                htn_cholesterol_treatment_factor,
                (if on_htn_meds {
                    covariates.htn_meds_systolic_bp_max_factor * systolic_bp_adjusted_max
                } else {
                    0.0
                }),
                (if on_cholesterol_meds {
                    covariates.cholesterol_meds_cholesterol_diff_factor * cholesterol_diff_factor
                } else {
                    0.0
                }),
                covariates.age_adjustment_cholesterol_diff_factor
                    * age_adjusted
                    * cholesterol_diff_factor,
                covariates.age_adjustment_hdl_cholesterol_diff_factor
                    * age_adjusted
                    * (covariates.cholesterol_base_multiplier * hdl_cholesterol - 1.3)
                    / 0.3,
                covariates.age_adjustment_systolic_bp_max_factor
                    * age_adjusted
                    * systolic_bp_adjusted_max,
                diabetes_age_factor,
                smoker_age_factor,
                covariates.age_min_egfr_factor * age_adjusted * (egfr.min(60.0) - 60.0) / -15.0,
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
            let covariates = Covariates::female_30_yr_ascvd();

            let calculation = vec![
                covariates.age_adjustment_factor * age_factor,
                covariates.constant,
                covariates.age_squared_factor * age_squared,
                covariates.total_cholesterol_diff_factor
                    * (covariates.cholesterol_base_multiplier * cholesterol_difference - 3.5),
                covariates.hdl_cholesterol_diff_factor
                    * (covariates.cholesterol_base_multiplier * hdl_cholesterol - 1.3)
                    / 0.3,
                covariates.systolic_bp_min_factor * (systolic_bp.min(110.0) - 110.0) / 20.0,
                covariates.systolic_bp_max_factor * (systolic_bp.max(110.0) - 130.0) / 20.0,
                if has_diabetes {
                    covariates.diabetes_factor
                } else {
                    0.0
                },
                if current_smoker {
                    covariates.smoker_factor
                } else {
                    0.0
                },
                covariates.egfr_min_factor * (egfr.min(60.0) - 60.0) / -15.0,
                covariates.egfr_max_factor * (egfr.max(60.0) - 90.0) / -15.0,
                if on_htn_meds {
                    covariates.htn_meds_factor
                } else {
                    0.0
                },
                if cholesterol_treated {
                    covariates.cholesterol_meds_factor
                } else {
                    0.0
                },
                if on_htn_meds {
                    covariates.htn_meds_systolic_bp_max_factor * (systolic_bp.max(110.0) - 130.0)
                        / 20.0
                } else {
                    0.0
                },
                if cholesterol_treated {
                    covariates.cholesterol_meds_cholesterol_diff_factor
                        * (covariates.cholesterol_base_multiplier * cholesterol_difference - 3.5)
                } else {
                    0.0
                },
                covariates.age_adjustment_cholesterol_diff_factor
                    * age_factor
                    * (covariates.cholesterol_base_multiplier * cholesterol_difference - 3.5),
                covariates.age_adjustment_hdl_cholesterol_diff_factor
                    * age_factor
                    * (covariates.cholesterol_base_multiplier * hdl_cholesterol - 1.3)
                    / 0.3,
                covariates.age_adjustment_systolic_bp_max_factor
                    * age_factor
                    * (systolic_bp.max(110.0) - 130.0)
                    / 20.0,
                if has_diabetes {
                    covariates.diabetes_age_factor * age_factor
                } else {
                    0.0
                },
                if current_smoker {
                    covariates.smoker_age_factor * age_factor
                } else {
                    0.0
                },
                covariates.age_min_egfr_factor * age_factor * (egfr.min(60.0) - 60.0) / -15.0,
            ]
            .iter()
            .sum();

            let risk_score = E.powf(calculation) / (1.0 + E.powf(calculation)) * 100.0;
            Ok(risk_score)
        }
        "male" => {
            let covariates = Covariates::male_30_yr_ascvd();

            let calculation = vec![
                covariates.age_adjustment_factor * age_factor,
                covariates.constant,
                covariates.age_squared_factor * age_squared,
                covariates.total_cholesterol_diff_factor
                    * (covariates.cholesterol_base_multiplier * cholesterol_difference - 3.5),
                covariates.hdl_cholesterol_diff_factor
                    * (covariates.cholesterol_base_multiplier * hdl_cholesterol - 1.3)
                    / 0.3,
                covariates.systolic_bp_min_factor * (systolic_bp.min(110.0) - 110.0) / 20.0,
                covariates.systolic_bp_max_factor * (systolic_bp.max(110.0) - 130.0) / 20.0,
                if has_diabetes {
                    covariates.diabetes_factor
                } else {
                    0.0
                },
                if current_smoker {
                    covariates.smoker_factor
                } else {
                    0.0
                },
                covariates.egfr_min_factor * (egfr.min(60.0) - 60.0) / -15.0,
                covariates.egfr_max_factor * (egfr.max(60.0) - 90.0) / -15.0,
                if on_htn_meds {
                    covariates.htn_meds_factor
                } else {
                    0.0
                },
                if cholesterol_treated {
                    covariates.cholesterol_meds_factor
                } else {
                    0.0
                },
                if on_htn_meds {
                    covariates.htn_meds_systolic_bp_max_factor * (systolic_bp.max(110.0) - 130.0)
                        / 20.0
                } else {
                    0.0
                },
                if cholesterol_treated {
                    covariates.cholesterol_meds_cholesterol_diff_factor
                        * (covariates.cholesterol_base_multiplier * cholesterol_difference - 3.5)
                } else {
                    0.0
                },
                covariates.age_adjustment_cholesterol_diff_factor
                    * age_factor
                    * (covariates.cholesterol_base_multiplier * cholesterol_difference - 3.5),
                covariates.age_adjustment_hdl_cholesterol_diff_factor
                    * age_factor
                    * (covariates.cholesterol_base_multiplier * hdl_cholesterol - 1.3)
                    / 0.3,
                covariates.age_adjustment_systolic_bp_max_factor
                    * age_factor
                    * (systolic_bp.max(110.0) - 130.0)
                    / 20.0,
                if has_diabetes {
                    covariates.diabetes_age_factor * age_factor
                } else {
                    0.0
                },
                if current_smoker {
                    covariates.smoker_age_factor * age_factor
                } else {
                    0.0
                },
                covariates.age_min_egfr_factor * age_factor * (egfr.min(60.0) - 60.0) / -15.0,
            ]
            .iter()
            .sum();

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
