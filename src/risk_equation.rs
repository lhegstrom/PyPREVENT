//! The PREVENT log-odds equation shared by every outcome and horizon.
//!
//! The literal centering and scaling values (55, 10, 110, 130, 20, 3.5, 1.3,
//! 0.3, 60, 90, -15, 25, 30, 5) come from the published equations (Khan et al.,
//! Circulation 2023); keep them inline so the code reads like the paper.
//! The term order of the final sum is part of the numerical contract.

use crate::model_coefficients::RiskModelCoefficients;
use crate::risk_factors::RiskFactors;

/// Returns the predicted risk as a percentage from 0 to 100.
pub(crate) fn calculate_risk_percentage(
    coefficients: &RiskModelCoefficients,
    risk_factors: &RiskFactors,
) -> f64 {
    let odds = calculate_log_odds(coefficients, risk_factors).exp();
    odds / (1.0 + odds) * 100.0
}

fn calculate_log_odds(coefficients: &RiskModelCoefficients, risk_factors: &RiskFactors) -> f64 {
    let cholesterol_diff = risk_factors.total_cholesterol - risk_factors.hdl_cholesterol;
    let adjusted_age = (risk_factors.age - 55.0) / 10.0;
    let adjusted_age_squared = adjusted_age.powi(2);

    let diabetes_term = if risk_factors.has_diabetes {
        coefficients.diabetes_factor
    } else {
        0.0
    };
    let smoker_term = if risk_factors.current_smoker {
        coefficients.smoker_factor
    } else {
        0.0
    };
    let htn_meds_term = if risk_factors.on_htn_meds {
        coefficients.htn_meds_factor
    } else {
        0.0
    };
    let cholesterol_meds_term = if risk_factors.on_cholesterol_meds {
        coefficients.cholesterol_meds_factor
    } else {
        0.0
    };
    let systolic_bp_adjusted_max = (risk_factors.systolic_bp.max(110.0) - 130.0) / 20.0;
    let systolic_bp_adjusted_min = (risk_factors.systolic_bp.min(110.0) - 110.0) / 20.0;
    let cholesterol_diff_factor = coefficients.cholesterol_base_multiplier * cholesterol_diff - 3.5;
    let hdl_cholesterol_diff_factor =
        (coefficients.cholesterol_base_multiplier * risk_factors.hdl_cholesterol - 1.3) / 0.3;

    let diabetes_age_term = if risk_factors.has_diabetes {
        coefficients.diabetes_age_factor * adjusted_age
    } else {
        0.0
    };
    let smoker_age_term = if risk_factors.current_smoker {
        coefficients.smoker_age_factor * adjusted_age
    } else {
        0.0
    };

    let egfr_adjusted_min = (risk_factors.egfr.min(60.0) - 60.0) / -15.0;
    let egfr_adjusted_max = (risk_factors.egfr.max(60.0) - 90.0) / -15.0;

    let bmi_adjusted_min = (risk_factors.bmi.min(30.0) - 25.0) / 5.0;
    let bmi_adjusted_max = (risk_factors.bmi.max(30.0) - 30.0) / 5.0;

    [
        coefficients.age_adjustment_factor * adjusted_age,
        coefficients.age_squared_factor * adjusted_age_squared,
        coefficients.constant,
        coefficients.total_cholesterol_diff_factor * cholesterol_diff_factor,
        coefficients.hdl_cholesterol_diff_factor * hdl_cholesterol_diff_factor,
        coefficients.systolic_bp_min_factor * systolic_bp_adjusted_min,
        coefficients.systolic_bp_max_factor * systolic_bp_adjusted_max,
        diabetes_term,
        smoker_term,
        coefficients.egfr_min_factor * egfr_adjusted_min,
        coefficients.egfr_max_factor * egfr_adjusted_max,
        htn_meds_term,
        cholesterol_meds_term,
        (if risk_factors.on_htn_meds {
            coefficients.htn_meds_systolic_bp_max_factor * systolic_bp_adjusted_max
        } else {
            0.0
        }),
        (if risk_factors.on_cholesterol_meds {
            coefficients.cholesterol_meds_cholesterol_diff_factor * cholesterol_diff_factor
        } else {
            0.0
        }),
        coefficients.age_adjustment_cholesterol_diff_factor
            * adjusted_age
            * cholesterol_diff_factor,
        coefficients.age_adjustment_hdl_cholesterol_diff_factor
            * adjusted_age
            * (coefficients.cholesterol_base_multiplier * risk_factors.hdl_cholesterol - 1.3)
            / 0.3,
        coefficients.age_adjustment_systolic_bp_max_factor
            * adjusted_age
            * systolic_bp_adjusted_max,
        diabetes_age_term,
        smoker_age_term,
        coefficients.age_min_egfr_factor * adjusted_age * egfr_adjusted_min,
        coefficients.bmi_min_factor * bmi_adjusted_min,
        coefficients.bmi_max_factor * bmi_adjusted_max,
        coefficients.age_bmi_max_factor * adjusted_age * bmi_adjusted_max,
    ]
    .iter()
    .sum()
}
