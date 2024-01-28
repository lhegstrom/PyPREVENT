pub(crate) struct Covariates {
    pub(crate) constant: f64,
    pub(crate) diabetes_factor: f64,
    pub(crate) smoker_factor: f64,
    pub(crate) htn_meds_factor: f64,
    pub(crate) cholesterol_meds_factor: f64,
    pub(crate) age_adjustment_factor: f64,
    pub(crate) age_squared_factor: f64,
    pub(crate) cholesterol_base_multiplier: f64,
    pub(crate) total_cholesterol_diff_factor: f64,
    pub(crate) hdl_cholesterol_diff_factor: f64,
    pub(crate) systolic_bp_min_factor: f64,
    pub(crate) systolic_bp_max_factor: f64,
    pub(crate) egfr_max_factor: f64,
    pub(crate) egfr_min_factor: f64,
    pub(crate) htn_meds_systolic_bp_max_factor: f64,
    pub(crate) cholesterol_meds_cholesterol_diff_factor: f64,
    pub(crate) age_adjustment_cholesterol_diff_factor: f64,
    pub(crate) age_adjustment_hdl_cholesterol_diff_factor: f64,
    pub(crate) age_adjustment_systolic_bp_max_factor: f64,
    pub(crate) age_min_egfr_factor: f64,
    pub(crate) diabetes_age_factor: f64,
    pub(crate) smoker_age_factor: f64,
}

impl Covariates {
    pub(crate) fn female_10_yr_ascvd() -> Covariates {
        Covariates {
            constant: -3.819975,
            diabetes_factor: 0.8348585,
            smoker_factor: 0.4831078,
            htn_meds_factor: 0.2265309,
            cholesterol_meds_factor: -0.0592374,
            age_adjustment_factor: 0.719883,
            cholesterol_base_multiplier: 0.02586,
            total_cholesterol_diff_factor: 0.1176967,
            hdl_cholesterol_diff_factor: -0.151185,
            systolic_bp_min_factor: -0.0835358,
            systolic_bp_max_factor: 0.3592852,
            egfr_min_factor: 0.4864619,
            egfr_max_factor: 0.0397779,
            htn_meds_systolic_bp_max_factor: -0.0395762,
            cholesterol_meds_cholesterol_diff_factor: 0.0844423,
            age_adjustment_cholesterol_diff_factor: -0.0567839,
            age_adjustment_hdl_cholesterol_diff_factor: 0.0325692,
            age_adjustment_systolic_bp_max_factor: -0.1035985,
            diabetes_age_factor: -0.2417542,
            smoker_age_factor: -0.0791142,
            age_min_egfr_factor: -0.1671492,
            age_squared_factor: 0.0,
        }
    }

    pub(crate) fn male_10_yr_ascvd() -> Covariates {
        Covariates {
            constant: -3.500655,
            diabetes_factor: 0.7189597,
            smoker_factor: 0.3956973,
            htn_meds_factor: 0.2036522,
            cholesterol_meds_factor: -0.0865581,
            age_adjustment_factor: 0.7099847,
            cholesterol_base_multiplier: 0.02586,
            total_cholesterol_diff_factor: 0.1658663,
            hdl_cholesterol_diff_factor: -0.1144285,
            systolic_bp_min_factor: -0.2837212,
            systolic_bp_max_factor: 0.3239977,
            egfr_min_factor: 0.3690075,
            egfr_max_factor: 0.0203619,
            htn_meds_systolic_bp_max_factor: -0.0322916,
            cholesterol_meds_cholesterol_diff_factor: 0.114563,
            age_adjustment_cholesterol_diff_factor: -0.0300005,
            age_adjustment_hdl_cholesterol_diff_factor: 0.0232747,
            age_adjustment_systolic_bp_max_factor: -0.0927024,
            diabetes_age_factor: -0.2018525,
            smoker_age_factor: -0.0970527,
            age_min_egfr_factor: -0.1217081,
            age_squared_factor: 0.0,
        }
    }

    pub(crate) fn female_30_yr_ascvd() -> Covariates {
        Covariates {
            constant: -1.974074,
            diabetes_factor: 0.6296707,
            smoker_factor: 0.268292,
            htn_meds_factor: 0.1875292,
            cholesterol_meds_factor: 0.0152476,
            age_adjustment_factor: 0.4669202,
            age_squared_factor: -0.0893118,
            cholesterol_base_multiplier: 0.02586,
            total_cholesterol_diff_factor: 0.1256901,
            hdl_cholesterol_diff_factor: -0.1542255,
            systolic_bp_min_factor: -0.0018093,
            systolic_bp_max_factor: 0.322949,
            egfr_min_factor: 0.100106,
            egfr_max_factor: 0.0499663,
            htn_meds_systolic_bp_max_factor: -0.0276123,
            cholesterol_meds_cholesterol_diff_factor: 0.0736147,
            age_adjustment_cholesterol_diff_factor: -0.0521962,
            age_adjustment_hdl_cholesterol_diff_factor: 0.0316918,
            age_adjustment_systolic_bp_max_factor: -0.1046101,
            diabetes_age_factor: -0.2727793,
            smoker_age_factor: -0.1530907,
            age_min_egfr_factor: -0.1299149,
        }
    }

    pub(crate) fn male_30_yr_ascvd() -> Covariates {
        Covariates {
            constant: -1.736444,
            diabetes_factor: 0.4790257,
            smoker_factor: 0.1782635,
            htn_meds_factor: 0.1421182,
            cholesterol_meds_factor: 0.0135996,
            age_adjustment_factor: 0.3994099,
            age_squared_factor: -0.0937484,
            cholesterol_base_multiplier: 0.02586,
            total_cholesterol_diff_factor: 0.1744643,
            hdl_cholesterol_diff_factor: -0.120203,
            systolic_bp_min_factor: -0.0665117,
            systolic_bp_max_factor: 0.2753037,
            egfr_min_factor: -0.0218789,
            egfr_max_factor: 0.0602553,
            htn_meds_systolic_bp_max_factor: -0.0218265,
            cholesterol_meds_cholesterol_diff_factor: 0.1013148,
            age_adjustment_cholesterol_diff_factor: -0.0312619,
            age_adjustment_hdl_cholesterol_diff_factor: 0.020673,
            age_adjustment_systolic_bp_max_factor: -0.0920935,
            diabetes_age_factor: -0.2159947,
            smoker_age_factor: -0.1548811,
            age_min_egfr_factor: -0.0712547,
        }
    }
}

/*

*/
