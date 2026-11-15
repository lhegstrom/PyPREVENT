//! Typed patient inputs: the sex that selects an equation, the prediction
//! horizon, and the risk factors that enter every equation.

use crate::config::{
    AGE_COLUMN, BMI_COLUMN, BMI_RANGE, CURRENT_SMOKER_COLUMN, EGFR_COLUMN, EGFR_RANGE,
    HAS_DIABETES_COLUMN, HDL_CHOLESTEROL_COLUMN, HDL_CHOLESTEROL_RANGE, ON_CHOLESTEROL_MEDS_COLUMN,
    ON_HTN_MEDS_COLUMN, SYSTOLIC_BP_COLUMN, SYSTOLIC_BP_RANGE, TEN_YEAR_AGE_RANGE,
    THIRTY_YEAR_AGE_RANGE, TOTAL_CHOLESTEROL_COLUMN, TOTAL_CHOLESTEROL_RANGE,
};
use pyo3::prelude::*;
use std::ops::RangeInclusive;
use std::str::FromStr;

/// Sex selects the sex-specific PREVENT equation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum Sex {
    Female,
    Male,
}

impl FromStr for Sex {
    type Err = String;

    /// Parses `"male"` or `"female"`, ignoring ASCII case.
    fn from_str(sex_label: &str) -> Result<Self, Self::Err> {
        if sex_label.eq_ignore_ascii_case("female") {
            Ok(Sex::Female)
        } else if sex_label.eq_ignore_ascii_case("male") {
            Ok(Sex::Male)
        } else {
            Err("Sex must be either 'male' or 'female'.".to_string())
        }
    }
}

impl Sex {
    /// Reads the batch encoding (0 = female, 1 = male); `None` for any other value.
    pub(crate) fn from_batch_flag(sex_flag: f64) -> Option<Self> {
        read_batch_flag(sex_flag).map(|is_male| if is_male { Sex::Male } else { Sex::Female })
    }
}

/// Prediction window of a PREVENT equation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum RiskHorizon {
    TenYear,
    ThirtyYear,
}

impl RiskHorizon {
    fn valid_age_range(self) -> RangeInclusive<f64> {
        match self {
            RiskHorizon::TenYear => TEN_YEAR_AGE_RANGE,
            RiskHorizon::ThirtyYear => THIRTY_YEAR_AGE_RANGE,
        }
    }
}

/// Risk factors that enter every PREVENT equation, in clinical units.
///
/// Python passes them as a dict whose keys match these field names, so a
/// misspelled or missing key fails loudly instead of shifting a value.
#[derive(Clone, Copy, Debug, FromPyObject)]
#[pyo3(from_item_all)]
pub(crate) struct RiskFactors {
    pub(crate) age: f64,
    pub(crate) total_cholesterol: f64,
    pub(crate) hdl_cholesterol: f64,
    pub(crate) systolic_bp: f64,
    pub(crate) has_diabetes: bool,
    pub(crate) current_smoker: bool,
    pub(crate) bmi: f64,
    pub(crate) egfr: f64,
    pub(crate) on_htn_meds: bool,
    pub(crate) on_cholesterol_meds: bool,
}

impl RiskFactors {
    /// Checks each measurement against its derivation range and reports the
    /// first one out of range, in the same order as earlier releases.
    pub(crate) fn validate_for(&self, horizon: RiskHorizon) -> Result<(), String> {
        require_within_range(self.age, horizon.valid_age_range(), "Age")?;
        require_within_range(
            self.total_cholesterol,
            TOTAL_CHOLESTEROL_RANGE,
            "Total cholesterol",
        )?;
        require_within_range(
            self.hdl_cholesterol,
            HDL_CHOLESTEROL_RANGE,
            "HDL cholesterol",
        )?;
        require_within_range(
            self.systolic_bp,
            SYSTOLIC_BP_RANGE,
            "Systolic blood pressure",
        )?;
        require_within_range(self.bmi, BMI_RANGE, "BMI")?;
        require_within_range(self.egfr, EGFR_RANGE, "eGFR")
    }

    /// Reads one batch row; `None` when a 0/1 flag column holds another value.
    pub(crate) fn from_batch_row(row: &[f64]) -> Option<Self> {
        Some(RiskFactors {
            age: row[AGE_COLUMN],
            total_cholesterol: row[TOTAL_CHOLESTEROL_COLUMN],
            hdl_cholesterol: row[HDL_CHOLESTEROL_COLUMN],
            systolic_bp: row[SYSTOLIC_BP_COLUMN],
            has_diabetes: read_batch_flag(row[HAS_DIABETES_COLUMN])?,
            current_smoker: read_batch_flag(row[CURRENT_SMOKER_COLUMN])?,
            bmi: row[BMI_COLUMN],
            egfr: row[EGFR_COLUMN],
            on_htn_meds: read_batch_flag(row[ON_HTN_MEDS_COLUMN])?,
            on_cholesterol_meds: read_batch_flag(row[ON_CHOLESTEROL_MEDS_COLUMN])?,
        })
    }
}

fn read_batch_flag(flag_value: f64) -> Option<bool> {
    if flag_value == 1.0 {
        Some(true)
    } else if flag_value == 0.0 {
        Some(false)
    } else {
        None
    }
}

fn require_within_range(
    value: f64,
    valid_range: RangeInclusive<f64>,
    input_label: &str,
) -> Result<(), String> {
    if valid_range.contains(&value) {
        Ok(())
    } else {
        Err(format!(
            "{input_label} must be between {} and {}",
            valid_range.start(),
            valid_range.end()
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALID_RISK_FACTORS: RiskFactors = RiskFactors {
        age: 40.0,
        total_cholesterol: 200.0,
        hdl_cholesterol: 50.0,
        systolic_bp: 120.0,
        has_diabetes: true,
        current_smoker: false,
        bmi: 25.0,
        egfr: 70.0,
        on_htn_meds: true,
        on_cholesterol_meds: false,
    };

    #[test]
    fn sex_parsing_ignores_case_and_rejects_other_labels() {
        assert_eq!("MALE".parse::<Sex>(), Ok(Sex::Male));
        assert_eq!("Female".parse::<Sex>(), Ok(Sex::Female));
        assert!(" male".parse::<Sex>().is_err());
        assert!("unknown".parse::<Sex>().is_err());
    }

    #[test]
    fn batch_flags_accept_only_zero_and_one() {
        assert_eq!(Sex::from_batch_flag(1.0), Some(Sex::Male));
        assert_eq!(Sex::from_batch_flag(0.0), Some(Sex::Female));
        for invalid_flag in [0.5, -1.0, 2.0, f64::NAN, f64::INFINITY] {
            assert_eq!(Sex::from_batch_flag(invalid_flag), None);
        }
    }

    #[test]
    fn batch_row_with_invalid_flag_is_rejected() {
        let mut row = [
            0.0, 40.0, 200.0, 50.0, 120.0, 1.0, 0.0, 25.0, 70.0, 1.0, 0.0,
        ];
        assert!(RiskFactors::from_batch_row(&row).is_some());
        row[ON_CHOLESTEROL_MEDS_COLUMN] = 0.5;
        assert!(RiskFactors::from_batch_row(&row).is_none());
    }

    #[test]
    fn validation_reports_first_invalid_measurement_with_range() {
        assert_eq!(
            VALID_RISK_FACTORS.validate_for(RiskHorizon::TenYear),
            Ok(())
        );
        let older_patient = RiskFactors {
            age: 65.0,
            ..VALID_RISK_FACTORS
        };
        assert_eq!(older_patient.validate_for(RiskHorizon::TenYear), Ok(()));
        assert_eq!(
            older_patient.validate_for(RiskHorizon::ThirtyYear),
            Err("Age must be between 30 and 59".to_string())
        );
        let two_invalid_values = RiskFactors {
            bmi: 18.4,
            egfr: 14.9,
            ..VALID_RISK_FACTORS
        };
        assert_eq!(
            two_invalid_values.validate_for(RiskHorizon::TenYear),
            Err("BMI must be between 18.5 and 39.9".to_string())
        );
    }
}
