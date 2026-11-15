"""Static values shared by the Python wrappers."""

# ============================================================
# Native Batch Array Layout
# ============================================================
# Column order of the (n, 11) array read by src/risk_calculation.rs; it must
# match the *_COLUMN indexes in src/config.rs.
NATIVE_BATCH_COLUMN_ORDER = (
    "sex",
    "age",
    "total_cholesterol",
    "hdl_cholesterol",
    "systolic_bp",
    "has_diabetes",
    "current_smoker",
    "bmi",
    "egfr",
    "on_htn_meds",
    "on_cholesterol_meds",
)
BOOLEAN_BATCH_COLUMNS = frozenset(
    {"has_diabetes", "current_smoker", "on_htn_meds", "on_cholesterol_meds"}
)
NATIVE_FLAG_BY_SEX_LABEL = {"female": 0, "male": 1}

# ============================================================
# Messages
# ============================================================
INVALID_ROWS_WARNING_TEMPLATE = (
    "WARNING: {invalid_row_count} patients were unable to have the score calculated"
    " as their input parameters were invalid or out of range."
)
