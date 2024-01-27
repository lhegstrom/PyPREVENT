use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
use std::f64;
use std::f64::consts::E;
use crate::utils::validate_input;


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

    validate_input(age, total_cholesterol, hdl_cholesterol, systolic_bp, bmi,  egfr, true)?;

    let cholesterol_diff = total_cholesterol - hdl_cholesterol;
    let age_adjusted = (age - 55.0) / 10.0;

    match sex.to_lowercase().as_str() {
        "female" => {
            let diabetes_factor = if has_diabetes { 0.8667604 } else { 0.0 };
            let smoker_factor = if current_smoker { 0.5360739 } else { 0.0 };
            let htn_meds_factor = if on_htn_meds { 0.3151672 } else { 0.0 };
            let htn_cholesterol_treatment_factor = if on_cholesterol_meds { 0.1477655 } else { 0.0 };
            let systolic_bp_adjusted_max = (systolic_bp.max(110.0) - 130.0) / 20.0;
            let cholesterol_diff_factor = 0.02586 * cholesterol_diff - 3.5;

            let diabetes_age_factor = if has_diabetes { -0.27057 * age_adjusted } else { 0.0 };
            let smoker_age_factor = if current_smoker { -0.078715 * age_adjusted } else { 0.0 };

            let calculation = 0.7939329 * age_adjusted
                - 3.307728

                + 0.0305239 * cholesterol_diff_factor
                - 0.1606857 * (0.02586 * hdl_cholesterol - 1.3) / 0.3

                - 0.2394003 * (systolic_bp.min(110.0) - 110.0) / 20.0
                + 0.3600781 * systolic_bp_adjusted_max

                + diabetes_factor
                + smoker_factor

                + 0.6045917 * (egfr.min(60.0) - 60.0) / -15.0
                + 0.0433769 * (egfr.max(60.0) - 90.0) / -15.0

                + htn_meds_factor
                - htn_cholesterol_treatment_factor

                + (if on_htn_meds { -0.0663612 * systolic_bp_adjusted_max } else { 0.0 })
                + (if on_cholesterol_meds { 0.1197879 * cholesterol_diff_factor } else { 0.0 })

                - 0.0819715 * age_adjusted * cholesterol_diff_factor
                + 0.0306769 * age_adjusted * (0.02586 * hdl_cholesterol - 1.3) / 0.3
                - 0.0946348 * age_adjusted * systolic_bp_adjusted_max

                + diabetes_age_factor
                + smoker_age_factor

                - 0.1637806 * age_adjusted * (egfr.min(60.0) - 60.0) / -15.0;

            let risk_score = E.powf(calculation) / (1.0 + E.powf(calculation)) * 100.0;
            Ok(risk_score)
        }
        "male" => {
            let diabetes_factor = if has_diabetes { 0.7692857 } else { 0.0 };
            let smoker_factor = if current_smoker { 0.4386871 } else { 0.0 };
            let htn_meds_factor = if on_htn_meds { 0.288879 } else { 0.0 };
            let htn_cholesterol_treatment_factor = if on_cholesterol_meds { 0.1337349 } else { 0.0 };
            let systolic_bp_adjusted_max = (systolic_bp.max(110.0) - 130.0) / 20.0;
            let cholesterol_diff_factor = 0.02586 * cholesterol_diff - 3.5;

            let diabetes_age_factor = if has_diabetes { 0.2251948 * age_adjusted } else { 0.0 };
            let smoker_age_factor = if current_smoker { 0.0895067 * age_adjusted } else { 0.0 };

            let calculation = 0.7688528 * age_adjusted
                - 3.031168

                + 0.0736174 * cholesterol_diff_factor
                - 0.0954431 * (0.02586 * hdl_cholesterol - 1.3) / 0.3

                - 0.4347345 * (systolic_bp.min(110.0) - 110.0) / 20.0
                + 0.3362658 * systolic_bp_adjusted_max


                + diabetes_factor
                + smoker_factor

                + 0.5378979 * (egfr.min(60.0) - 60.0) / -15.0
                + 0.0164827 * (egfr.max(60.0) - 90.0) / -15.0

                + htn_meds_factor
                - htn_cholesterol_treatment_factor

                - (if on_htn_meds { 0.0475924 * systolic_bp_adjusted_max } else { 0.0 })
                + (if on_cholesterol_meds { 0.150273 * cholesterol_diff_factor } else { 0.0 })
                - 0.0517874 * age_adjusted * cholesterol_diff_factor
                + 0.0191169 * age_adjusted * (0.02586 * hdl_cholesterol - 1.3) / 0.3
                - 0.1049477 * age_adjusted * systolic_bp_adjusted_max

                - diabetes_age_factor
                - smoker_age_factor

                - 0.1543702 * age_adjusted * (egfr.min(60.0) - 60.0) / -15.0;

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
    cholesterol_treated: bool,
) -> Result<f64, String> {

    validate_input(age, total_cholesterol, hdl_cholesterol, systolic_bp, bmi,  egfr, false)?;

    let cholesterol_difference = total_cholesterol - hdl_cholesterol;
    let age_factor = (age - 55.0) / 10.0;
    let age_squared = age_factor.powi(2);

    match sex.to_lowercase().as_str() {
        "female" => {
            let calculation = 0.5503079 * age_factor
                - 1.318827
                - 0.0928369 * age_squared
                + 0.0409794 * (0.02586 * cholesterol_difference - 3.5)
                - 0.1663306 * (0.02586 * hdl_cholesterol - 1.3) / 0.3
                - 0.1628654 * (systolic_bp.min(110.0) - 110.0) / 20.0
                + 0.3299505 * (systolic_bp.max(110.0) - 130.0) / 20.0
                + if has_diabetes { 0.6793894 } else { 0.0 }
                + if current_smoker { 0.3196112 } else { 0.0 }
                + 0.1857101 * (egfr.min(60.0) - 60.0) / -15.0
                + 0.0553528 * (egfr.max(60.0) - 90.0) / -15.0
                + if on_htn_meds { 0.2894 } else { 0.0 }
                - if cholesterol_treated { 0.075688 } else { 0.0 }
                - if on_htn_meds { 0.056367 * (systolic_bp.max(110.0) - 130.0) / 20.0 } else { 0.0 }
                + if cholesterol_treated { 0.1071019 * (0.02586 * cholesterol_difference - 3.5) } else { 0.0 }
                - 0.0751438 * age_factor * (0.02586 * cholesterol_difference - 3.5)
                + 0.0301786 * age_factor * (0.02586 * hdl_cholesterol - 1.3) / 0.3
                - 0.0998776 * age_factor * (systolic_bp.max(110.0) - 130.0) / 20.0
                - if has_diabetes { 0.3206166 * age_factor } else { 0.0 }
                - if current_smoker { 0.1607862 * age_factor } else { 0.0 }
                - 0.1450788 * age_factor * (egfr.min(60.0) - 60.0) / -15.0;

            let risk_score = E.powf(calculation) / (1.0 + E.powf(calculation)) * 100.0;
            Ok(risk_score)
        }
        "male" => {
            let calculation = 0.4627309 * age_factor
                - 1.148204
                - 0.0984281 * age_squared

                + 0.0836088 * (0.02586 * cholesterol_difference - 3.5)
                - 0.1029824 * (0.02586 * hdl_cholesterol - 1.3) / 0.3

                - 0.2140352 * (systolic_bp.min(110.0) - 110.0) / 20.0
                + 0.2904325 * (systolic_bp.max(110.0) - 130.0) / 20.0

                + if has_diabetes { 0.5331276 } else { 0.0 }
                + if current_smoker { 0.2141914 } else { 0.0 }

                + 0.1155556 * (egfr.min(60.0) - 60.0) / -15.0
                + 0.0603775 * (egfr.max(60.0) - 90.0) / -15.0

                + if on_htn_meds { 0.232714 } else { 0.0 }
                - if cholesterol_treated { 0.0272112 } else { 0.0 }

                - if on_htn_meds { 0.0384488 * (systolic_bp.max(110.0) - 130.0) / 20.0 } else { 0.0 }
                + if cholesterol_treated { 0.134192 * (0.02586 * cholesterol_difference - 3.5) } else { 0.0 }

                - 0.0511759 * age_factor * (0.02586 * cholesterol_difference - 3.5)
                + 0.0165865 * age_factor * (0.02586 * hdl_cholesterol - 1.3) / 0.3
                - 0.1101437 * age_factor * (systolic_bp.max(110.0) - 130.0) / 20.0
                - if has_diabetes { 0.2585943 * age_factor } else { 0.0 }
                - if current_smoker { 0.1566406 * age_factor } else { 0.0 }
                - 0.1166776 * age_factor * (egfr.min(60.0) - 60.0) / -15.0;

            let risk_score = E.powf(calculation) / (1.0 + E.powf(calculation)) * 100.0;
            Ok(risk_score)
        }
        _ => Err("Sex must be either 'male' or 'female'.".to_string()),
    }
}




#[pyfunction]
pub fn calculate_10_yr_cvd_rust(sex: String, age: f64, total_cholesterol: f64, hdl_cholesterol: f64,
                                    systolic_bp: f64, has_diabetes: bool, current_smoker: bool, bmi: f64,
                                    egfr: f64, on_htn_meds: bool, on_cholesterol_meds: bool) -> PyResult<f64> {
    match calculate_10_yr_cvd_risk(&sex, age, total_cholesterol, hdl_cholesterol, systolic_bp, has_diabetes, current_smoker, bmi, egfr, on_htn_meds, on_cholesterol_meds) {
        Ok(value) => Ok(value),
        Err(e) => Err(PyValueError::new_err(e)), // Convert Rust String error to Python ValueError
    }
}

#[pyfunction]
pub fn calculate_30_yr_cvd_rust(sex: String, age: f64, total_cholesterol: f64, hdl_cholesterol: f64,
                                    systolic_bp: f64, has_diabetes: bool, current_smoker: bool, bmi: f64,
                                    egfr: f64, on_htn_meds: bool, on_cholesterol_meds: bool) -> PyResult<f64> {
    match calculate_30_yr_cvd_risk(&sex, age, total_cholesterol, hdl_cholesterol, systolic_bp, has_diabetes, current_smoker, bmi, egfr, on_htn_meds, on_cholesterol_meds) {
        Ok(value) => Ok(value),
        Err(e) => Err(PyValueError::new_err(e)), // Convert Rust String error to Python ValueError
    }
}
