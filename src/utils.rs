use crate::covariates::Covariates;
use std::f64;

pub fn validate_input(
    age: f64,
    total_cholesterol: f64,
    hdl_cholesterol: f64,
    systolic_bp: f64,
    bmi: f64,
    egfr: f64,
    ten_year: bool,
) -> Result<(), String> {
    if ten_year {
        validate_10_yr_age(&age)?
    } else {
        validate_30_yr_age(&age)?
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
    Ok(())
}

fn validate_10_yr_age(&age: &f64) -> Result<(), String> {
    if !(30.0..=79.0).contains(&age) {
        return Err("Age must be between 30 and 79".to_string());
    }
    Ok(())
}

fn validate_30_yr_age(&age: &f64) -> Result<(), String> {
    if !(30.0..=59.0).contains(&age) {
        return Err("Age must be between 30 and 59".to_string());
    }
    Ok(())
}

pub fn common_calculation(
    covariates: &Covariates,
    has_diabetes: bool,
    current_smoker: bool,
    on_htn_meds: bool,
    on_cholesterol_meds: bool,
    systolic_bp: f64,
    cholesterol_diff: f64,
    hdl_cholesterol: f64,
    adjusted_age: f64,
    adjusted_age_squared: f64,
    egfr: f64,
    bmi: f64,
) -> f64 {
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
    let systolic_bp_adjusted_min = (systolic_bp.min(110.0) - 110.0) / 20.0;
    let cholesterol_diff_factor = covariates.cholesterol_base_multiplier * cholesterol_diff - 3.5;
    let hdl_cholesterol_diff_factor =
        (covariates.cholesterol_base_multiplier * hdl_cholesterol - 1.3) / 0.3;

    let diabetes_age_factor = if has_diabetes {
        covariates.diabetes_age_factor * adjusted_age
    } else {
        0.0
    };
    let smoker_age_factor = if current_smoker {
        covariates.smoker_age_factor * adjusted_age
    } else {
        0.0
    };

    let egfr_adjusted_min = (egfr.min(60.0) - 60.0) / -15.0;
    let egfr_adjusted_max = (egfr.max(60.0) - 90.0) / -15.0;

    let bmi_adjusted_min = (bmi.min(30.0) - 25.0) / 5.0;
    let bmi_adjusted_max = (bmi.max(30.0) - 30.0) / 5.0;

    vec![
        covariates.age_adjustment_factor * adjusted_age,
        covariates.age_squared_factor * adjusted_age_squared,
        covariates.constant,
        covariates.total_cholesterol_diff_factor * cholesterol_diff_factor,
        covariates.hdl_cholesterol_diff_factor * hdl_cholesterol_diff_factor,
        covariates.systolic_bp_min_factor * systolic_bp_adjusted_min,
        covariates.systolic_bp_max_factor * systolic_bp_adjusted_max,
        diabetes_factor,
        smoker_factor,
        covariates.egfr_min_factor * egfr_adjusted_min,
        covariates.egfr_max_factor * egfr_adjusted_max,
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
        covariates.age_adjustment_cholesterol_diff_factor * adjusted_age * cholesterol_diff_factor,
        covariates.age_adjustment_hdl_cholesterol_diff_factor
            * adjusted_age
            * (covariates.cholesterol_base_multiplier * hdl_cholesterol - 1.3)
            / 0.3,
        covariates.age_adjustment_systolic_bp_max_factor * adjusted_age * systolic_bp_adjusted_max,
        diabetes_age_factor,
        smoker_age_factor,
        covariates.age_min_egfr_factor * adjusted_age * egfr_adjusted_min,
        covariates.bmi_min_factor * bmi_adjusted_min,
        covariates.bmi_max_factor * bmi_adjusted_max,
        covariates.age_bmi_max_factor * adjusted_age * bmi_adjusted_max,
    ]
    .iter()
    .sum()
}
