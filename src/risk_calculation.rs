//! Runs one outcome's equation for a single Python patient or a NumPy batch.

use crate::config::{BATCH_COLUMN_COUNT, MINIMUM_PARALLEL_BATCH_ROWS, SEX_COLUMN};
use crate::model_coefficients::RiskModelCoefficients;
use crate::risk_equation::calculate_risk_percentage;
use crate::risk_factors::{RiskFactors, RiskHorizon, Sex};
use numpy::{PyArray, PyReadonlyArrayDyn, PyUntypedArrayMethods};
use pyo3::exceptions::PyValueError;
use pyo3::prelude::*;
use rayon::prelude::*;

/// Picks an outcome's published coefficients for one sex and horizon.
pub(crate) type CoefficientSelector = fn(Sex, RiskHorizon) -> &'static RiskModelCoefficients;

/// Validates one patient and returns their risk percentage, or a `ValueError`.
pub(crate) fn calculate_patient_risk(
    sex_label: &str,
    risk_factors: &RiskFactors,
    horizon: RiskHorizon,
    select_coefficients: CoefficientSelector,
) -> PyResult<f64> {
    risk_factors
        .validate_for(horizon)
        .and_then(|()| sex_label.parse::<Sex>())
        .map(|sex| calculate_risk_percentage(select_coefficients(sex, horizon), risk_factors))
        .map_err(PyValueError::new_err)
}

/// Scores every row of an `(n, 11)` array in input order; invalid rows become NaN.
pub(crate) fn calculate_batch_risk(
    py: Python<'_>,
    data: PyReadonlyArrayDyn<'_, f64>,
    horizon: RiskHorizon,
    select_coefficients: CoefficientSelector,
) -> PyResult<Py<PyAny>> {
    let shape = data.shape();
    if shape.len() != 2 || shape[1] != BATCH_COLUMN_COUNT {
        return Err(PyValueError::new_err("Array shape must be (n, 11)"));
    }

    // Own the input before releasing the GIL: Python may mutate its array while
    // other threads run. Iteration preserves logical row order for strided input.
    let rows: Vec<f64> = if data.is_c_contiguous() {
        data.as_slice()?.to_vec()
    } else {
        let view = data
            .as_array()
            .into_dimensionality::<numpy::ndarray::Ix2>()
            .expect("two-dimensional shape was validated above");
        let mut rows = Vec::with_capacity(view.len());
        for row in view.outer_iter() {
            rows.extend(row.iter().copied());
        }
        rows
    };
    drop(data);

    let calculate_row_risk = |row: &[f64]| -> f64 {
        let Some(sex) = Sex::from_batch_flag(row[SEX_COLUMN]) else {
            return f64::NAN;
        };
        let Some(risk_factors) = RiskFactors::from_batch_row(row) else {
            return f64::NAN;
        };
        match risk_factors.validate_for(horizon) {
            Ok(()) => calculate_risk_percentage(select_coefficients(sex, horizon), &risk_factors),
            Err(_) => f64::NAN,
        }
    };
    let risk_percentages: Vec<f64> = py.detach(|| {
        if rows.len() < MINIMUM_PARALLEL_BATCH_ROWS * BATCH_COLUMN_COUNT {
            rows.chunks_exact(BATCH_COLUMN_COUNT)
                .map(calculate_row_risk)
                .collect()
        } else {
            rows.par_chunks_exact(BATCH_COLUMN_COUNT)
                .map(calculate_row_risk)
                .collect()
        }
    });

    Ok(PyArray::from_vec(py, risk_percentages).into_any().unbind())
}
