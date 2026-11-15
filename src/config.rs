//! Static limits and layout values shared by the scalar and batch paths.

use std::ops::RangeInclusive;

// ============================================================
// Valid Input Ranges (PREVENT derivation limits)
// ============================================================
pub(crate) const TEN_YEAR_AGE_RANGE: RangeInclusive<f64> = 30.0..=79.0;
pub(crate) const THIRTY_YEAR_AGE_RANGE: RangeInclusive<f64> = 30.0..=59.0;
pub(crate) const TOTAL_CHOLESTEROL_RANGE: RangeInclusive<f64> = 130.0..=320.0;
pub(crate) const HDL_CHOLESTEROL_RANGE: RangeInclusive<f64> = 20.0..=100.0;
pub(crate) const SYSTOLIC_BP_RANGE: RangeInclusive<f64> = 90.0..=200.0;
pub(crate) const BMI_RANGE: RangeInclusive<f64> = 18.5..=39.9;
pub(crate) const EGFR_RANGE: RangeInclusive<f64> = 15.0..=140.0;

// ============================================================
// Batch Array Layout (must match NATIVE_BATCH_COLUMN_ORDER in pyprevent/config.py)
// ============================================================
pub(crate) const BATCH_COLUMN_COUNT: usize = 11;
pub(crate) const SEX_COLUMN: usize = 0;
pub(crate) const AGE_COLUMN: usize = 1;
pub(crate) const TOTAL_CHOLESTEROL_COLUMN: usize = 2;
pub(crate) const HDL_CHOLESTEROL_COLUMN: usize = 3;
pub(crate) const SYSTOLIC_BP_COLUMN: usize = 4;
pub(crate) const HAS_DIABETES_COLUMN: usize = 5;
pub(crate) const CURRENT_SMOKER_COLUMN: usize = 6;
pub(crate) const BMI_COLUMN: usize = 7;
pub(crate) const EGFR_COLUMN: usize = 8;
pub(crate) const ON_HTN_MEDS_COLUMN: usize = 9;
pub(crate) const ON_CHOLESTEROL_MEDS_COLUMN: usize = 10;

// ============================================================
// Parallelism
// ============================================================
/// Smaller batches run serially: scheduling a Rayon task costs more than the work.
pub(crate) const MINIMUM_PARALLEL_BATCH_ROWS: usize = 256;
