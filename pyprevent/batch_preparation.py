"""Convert a patient DataFrame into the numeric array the native batch code reads."""

import numpy as np
import pandas as pd

from .config import (
    BOOLEAN_BATCH_COLUMNS,
    INVALID_ROWS_WARNING_TEMPLATE,
    NATIVE_BATCH_COLUMN_ORDER,
    NATIVE_FLAG_BY_SEX_LABEL,
)


def _build_native_batch_array(
    df: pd.DataFrame,
    sex: str = "sex",
    age: str = "age",
    total_cholesterol: str = "total_cholesterol",
    hdl_cholesterol: str = "hdl_cholesterol",
    systolic_bp: str = "systolic_bp",
    has_diabetes: str = "has_diabetes",
    current_smoker: str = "current_smoker",
    bmi: str = "bmi",
    egfr: str = "egfr",
    on_htn_meds: str = "on_htn_meds",
    on_cholesterol_meds: str = "on_cholesterol_meds",
    **unexpected_column_mappings: str,
) -> np.ndarray:
    """Build the (n, 11) float64 array for the native batch functions.

    Each keyword names the DataFrame column that holds that input. Invalid
    sex labels and non-0/1 boolean values become NaN, so the native code
    rejects those rows instead of scoring them.

    Raises:
        TypeError: If an unknown column mapping is supplied.
        ValueError: If column labels are duplicated or a mapped column is missing.
    """
    column_name_by_parameter = {
        "sex": sex,
        "age": age,
        "total_cholesterol": total_cholesterol,
        "hdl_cholesterol": hdl_cholesterol,
        "systolic_bp": systolic_bp,
        "has_diabetes": has_diabetes,
        "current_smoker": current_smoker,
        "bmi": bmi,
        "egfr": egfr,
        "on_htn_meds": on_htn_meds,
        "on_cholesterol_meds": on_cholesterol_meds,
    }

    if unexpected_column_mappings:
        raise TypeError(
            f"Unexpected column mappings: {', '.join(sorted(unexpected_column_mappings))}"
        )
    if not df.columns.is_unique:
        raise ValueError("DataFrame column names must be unique.")
    for parameter_name in NATIVE_BATCH_COLUMN_ORDER:
        column_name = column_name_by_parameter[parameter_name]
        if column_name not in df.columns:
            raise ValueError(
                f"Column '{column_name}' for parameter '{parameter_name}' not found in DataFrame."
            )

    # Allocate the native layout once, without copying the whole DataFrame.
    native_batch_array = np.empty(
        (len(df), len(NATIVE_BATCH_COLUMN_ORDER)), dtype=np.float64
    )
    for column_index, parameter_name in enumerate(NATIVE_BATCH_COLUMN_ORDER):
        column_values = df[column_name_by_parameter[parameter_name]]
        if parameter_name == "sex":
            try:
                column_values = column_values.str.lower().map(NATIVE_FLAG_BY_SEX_LABEL)
            except AttributeError:
                # An entirely numeric or missing column has no string accessor.
                native_batch_array[:, column_index] = np.nan
                continue
        elif parameter_name in BOOLEAN_BATCH_COLUMNS and not pd.api.types.is_bool_dtype(
            column_values.dtype
        ):
            column_values = column_values.where(column_values.isin([0, 1]))
        if not pd.api.types.is_numeric_dtype(column_values.dtype):
            column_values = pd.to_numeric(column_values, errors="coerce")
        native_batch_array[:, column_index] = column_values.to_numpy(
            dtype=np.float64, na_value=np.nan
        )
    return native_batch_array


def _report_invalid_row_count(risk_percentages: np.ndarray) -> None:
    """Print one warning line when any row could not be scored."""
    invalid_row_count = np.isnan(risk_percentages).sum()
    if invalid_row_count > 0:
        print(INVALID_ROWS_WARNING_TEMPLATE.format(invalid_row_count=invalid_row_count))
