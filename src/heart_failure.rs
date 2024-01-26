use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;
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
    // Input validation
    if !(30.0..=79.0).contains(&age) {
        return Err("Age must be between 30 and 79".to_string());
    }
    if !(130.0..=320.0).contains(&total_cholesterol) {
        return Err("Total cholesterol must be between 130 and 320".to_string());
    }
    if !(20.0..=100.0).contains(&hdl_cholesterol) {
        return Err("HDL cholesterol must be between 20 and 100".to_string());
    }
    if !(90.0..=200.0).contains(&systolic_blood_pressure) {
        return Err("Systolic blood pressure must be between 90 and 200".to_string());
    }
    if !(18.5..=39.9).contains(&body_mass_index) {
        return Err("BMI must be between 18.5 and 39.9".to_string());
    }
    if !(15.0..=140.0).contains(&estimated_glomerular_filtration_rate) {
        return Err("eGFR must be between 15 and 140".to_string());
    }


    let cholesterol_diff = total_cholesterol - hdl_cholesterol;
    let age_adjustment_factor = ((age - 55.0) / 10.0).powi(2);

    let sex_lower = sex.to_lowercase();
    match sex_lower.as_str() {
        "female" => {
            let diabetes_factor = if has_diabetes { 1.0 } else { 0.0 };
            let smoker_factor = if is_current_smoker { 0.583916 } else { 0.0 };
            let htn_meds_factor = if on_hypertension_meds { 0.3534442 } else { 0.0 };
            let systolic_bp_adjustment = if on_hypertension_meds {
                0.0981511 * (systolic_blood_pressure.max(110.0) - 130.0) / 20.0
            } else {
                0.0
            };

            let diabetes_age_factor = if has_diabetes { 0.3581041 * (age - 55.0) / 10.0 } else { 0.0 };
            let smoker_age_factor = if is_current_smoker { 0.1159453 * (age - 55.0) / 10.0 } else { 0.0 };
            let total_heart_failure_value = 0.8998235 * (age - 55.0) / 10.0 - 4.310409 - 0.4559771 * (systolic_blood_pressure.min(110.0) - 110.0) / 20.0 + 0.3576505 * (systolic_blood_pressure.max(110.0) - 130.0) / 20.0 + diabetes_factor + smoker_factor - 0.0072294 * (body_mass_index.min(30.0) - 25.0) / 5.0 + 0.2997706 * (body_mass_index.max(30.0) - 30.0) / 5.0 + 0.7451638 * (estimated_glomerular_filtration_rate.min(60.0) - 60.0) / -15.0 + 0.0557087 * (estimated_glomerular_filtration_rate.max(60.0) - 90.0) / -15.0 + htn_meds_factor - systolic_bp_adjustment - 0.0946663 * (age - 55.0) / 10.0 * (systolic_blood_pressure.max(110.0) - 130.0) / 20.0 - diabetes_age_factor - smoker_age_factor - 0.003878 * (age - 55.0) / 10.0 * (body_mass_index.max(30.0) - 30.0) / 5.0 - 0.1884289 * (age - 55.0) / 10.0 * (estimated_glomerular_filtration_rate.min(60.0) - 60.0) / -15.0;

            let risk_value = E.powf(total_heart_failure_value) / (1.0 + E.powf(total_heart_failure_value)) * 100.0;
            Ok(risk_value)
        }
        "male" => {
            let diabetes_factor_male = if has_diabetes { 0.923776 } else { 0.0 };
            let smoker_factor_male = if is_current_smoker { 0.5023736 } else { 0.0 };
            let htn_meds_factor_male = if on_hypertension_meds { 0.2980922 } else { 0.0 };
            let systolic_bp_adjustment_male = if on_hypertension_meds {
                0.0497731 * (systolic_blood_pressure.max(110.0) - 130.0) / 20.0
            } else {
                0.0
            };

            let diabetes_age_factor_male = if has_diabetes { 0.3040924 * (age - 55.0) / 10.0 } else { 0.0 };
            let smoker_age_factor_male = if is_current_smoker { 0.1401688 * (age - 55.0) / 10.0 } else { 0.0 };
            let total_heart_failure_value_male = (age - 55.0) / 10.0 * 0.8972642 - 3.946391 - 0.6811466 * (systolic_blood_pressure.min(110.0) - 110.0) / 20.0 + 0.3634461 * (systolic_blood_pressure.max(110.0) - 130.0) / 20.0 + diabetes_factor_male + smoker_factor_male - (body_mass_index.min(30.0) - 25.0) / 5.0 * 0.0485841 + (body_mass_index.max(30.0) - 30.0) / 5.0 * 0.3726929 + (estimated_glomerular_filtration_rate.min(60.0) - 60.0) / -15.0 * 0.6926917 + (estimated_glomerular_filtration_rate.max(60.0) - 90.0) / -15.0 * 0.0251827 + htn_meds_factor_male - systolic_bp_adjustment_male - (age - 55.0) / 10.0 * 0.1289201 * ((systolic_blood_pressure.max(110.0) - 130.0) / 20.0) - diabetes_age_factor_male - smoker_age_factor_male + (age - 55.0) / 10.0 * 0.0068126 * ((body_mass_index.max(30.0) - 30.0) / 5.0) - (age - 55.0) / 10.0 * 0.1797778 * ((estimated_glomerular_filtration_rate.min(60.0) - 60.0) / -15.0);

            let risk_value = E.powf(total_heart_failure_value_male) / (1.0 + E.powf(total_heart_failure_value_male)) * 100.0;
            Ok(risk_value)
        }
        _ => Err("Sex must be either 'male' or 'female'.".to_string()),
    }
}



pub fn calculate_thirty_year_heart_failure(sex: &str,
    age: f64,
    total_cholesterol: f64,
    hdl_cholesterol: f64,
    systolic_bp: f64,
    diabetes: bool,
    smoker: bool,
    bmi: f64,
    egfr: f64,
    on_htn_meds: bool,
    _cholesterol_treated: bool,) -> Result<f64, String> {
    // Input validation
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


    let cholesterol_diff = total_cholesterol - hdl_cholesterol;
    let age_factor = (age - 55.0) / 10.0;
    let age_factor_squared = age_factor.powi(2);
    let bmi_30_factor = (bmi.max(30.0) - 30.0) / 5.0;
    let egfr_60_factor = (egfr.min(60.0) - 60.0) / -15.0;
    let systolic_bp_110_factor = (systolic_bp.min(110.0) - 110.0) / 20.0;
    let systolic_bp_130_factor = (systolic_bp.max(110.0) - 130.0) / 20.0;

    match sex.to_lowercase().as_str() {
        "female" => {
            let diabetes_factor = if diabetes { 0.8330787 } else { 0.0 };
            let smoker_factor = if smoker { 0.3438651 } else { 0.0 };
            let htn_meds_factor = if on_htn_meds { 0.333921 } else { 0.0 };
            let htn_meds_bp_factor = if on_htn_meds { 0.0893177 * systolic_bp_130_factor } else { 0.0 };
            let diabetes_age_factor = if diabetes { 0.404855 * age_factor } else { 0.0 };
            let smoker_age_factor = if smoker { 0.1982991 * age_factor } else { 0.0 };

            let e = 0.6254374 * age_factor
                - 2.205379
                - 0.0983038 * age_factor_squared
                - 0.3919241 * systolic_bp_110_factor
                + 0.3142295 * systolic_bp_130_factor
                + diabetes_factor
                + smoker_factor
                + 0.0594874 * (bmi.min(30.0) - 25.0) / 5.0
                + 0.2525536 * bmi_30_factor
                + 0.2981642 * egfr_60_factor
                + 0.0667159 * (egfr.max(60.0) - 90.0) / -15.0
                + htn_meds_factor
                - htn_meds_bp_factor
                - 0.0974299 * age_factor * systolic_bp_130_factor
                - diabetes_age_factor
                - smoker_age_factor
                - 0.0035619 * age_factor * bmi_30_factor
                - 0.1564215 * age_factor * egfr_60_factor;

            Ok(E.powf(e) / (1.0 + E.powf(e)) * 100.0)

        }
        "male" => {
            let diabetes_factor = if diabetes { 0.6840338 } else { 0.0 };
            let smoker_factor = if smoker { 0.2656273 } else { 0.0 };
            let htn_meds_factor = if on_htn_meds { 0.2583631 } else { 0.0 };
            let htn_meds_bp_factor = if on_htn_meds { 0.0391938 * systolic_bp_130_factor } else { 0.0 };
            let diabetes_age_factor = if diabetes { 0.3273572 * age_factor } else { 0.0 };
            let smoker_age_factor = if smoker { 0.2043019 * age_factor } else { 0.0 };

            let e = 0.5681541 * age_factor
                - 1.95751
                - 0.1048388 * age_factor_squared
                - 0.4761564 * systolic_bp_110_factor
                + 0.30324 * systolic_bp_130_factor
                + diabetes_factor
                + smoker_factor
                + 0.0833107 * (bmi.min(30.0) - 25.0) / 5.0
                + 0.26999 * bmi_30_factor
                + 0.2541805 * egfr_60_factor
                + 0.0638923 * (egfr.max(60.0) - 90.0) / -15.0
                + htn_meds_factor
                - htn_meds_bp_factor
                - 0.1269124 * age_factor * systolic_bp_130_factor
                - diabetes_age_factor
                - smoker_age_factor
                - 0.0182831 * age_factor * bmi_30_factor
                - 0.1342618 * age_factor * egfr_60_factor;
            Ok(E.powf(e) / (1.0 + E.powf(e)) * 100.0)
        }
        _ => Err("Sex must be either 'male' or 'female'.".to_string()),
    }
}

#[pyfunction]
pub fn calculate_10_yr_heart_failure_rust(sex: String, age: f64, total_cholesterol: f64, hdl_cholesterol: f64,
                                    systolic_bp: f64, has_diabetes: bool, is_smoker: bool, bmi: f64,
                                    egfr: f64, on_meds: bool, _cholesterol_treated: bool) -> PyResult<f64> {
    match calculate_10_yr_heart_failure_risk(&sex, age, total_cholesterol, hdl_cholesterol, systolic_bp, has_diabetes, is_smoker, bmi, egfr, on_meds, _cholesterol_treated) {
        Ok(value) => Ok(value),
        Err(e) => Err(PyValueError::new_err(e)), // Convert Rust String error to Python ValueError
    }
}

#[pyfunction]
pub fn calculate_30_yr_heart_failure_rust(sex: String, age: f64, total_cholesterol: f64, hdl_cholesterol: f64,
                                    systolic_bp: f64, has_diabetes: bool, is_smoker: bool, bmi: f64,
                                    egfr: f64, on_meds: bool, _cholesterol_treated: bool) -> PyResult<f64> {
    match calculate_thirty_year_heart_failure(&sex, age, total_cholesterol, hdl_cholesterol, systolic_bp, has_diabetes, is_smoker, bmi, egfr, on_meds, _cholesterol_treated) {
        Ok(value) => Ok(value),
        Err(e) => Err(PyValueError::new_err(e)), // Convert Rust String error to Python ValueError
    }
}