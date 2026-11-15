import numpy as np
import pandas as pd


def _prepare_df_for_batch(
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
    **kwargs,
) -> np.ndarray:
    # Default column names
    column_mapping = {
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

    if kwargs:
        raise TypeError(f"Unexpected column mappings: {', '.join(sorted(kwargs))}")
    if not df.columns.is_unique:
        raise ValueError("DataFrame column names must be unique.")

    # Ensure all required column names exist in the DataFrame
    for key, col in column_mapping.items():
        if col not in df.columns:
            raise ValueError(
                f"Column '{col}' for parameter '{key}' not found in DataFrame."
            )

    # Allocate the native layout once, without copying the whole DataFrame.
    # Invalid categories remain NaN so the native validator can reject the row.
    data = np.empty((len(df), len(column_mapping)), dtype=np.float64)
    bool_keys = {"has_diabetes", "current_smoker", "on_htn_meds", "on_cholesterol_meds"}
    for index, (key, column) in enumerate(column_mapping.items()):
        values = df[column]
        if key == "sex":
            try:
                values = values.str.lower().map({"female": 0, "male": 1})
            except AttributeError:
                # An entirely numeric/missing column has no string accessor.
                data[:, index] = np.nan
                continue
        elif key in bool_keys and not pd.api.types.is_bool_dtype(values.dtype):
            values = values.where(values.isin([0, 1]))
        if not pd.api.types.is_numeric_dtype(values.dtype):
            values = pd.to_numeric(values, errors="coerce")
        data[:, index] = values.to_numpy(dtype=np.float64, na_value=np.nan)
    return data


def _report_any_null_values(result: np.ndarray) -> None:
    null_rows = np.isnan(result).sum()
    if null_rows > 0:
        print(
            f"WARNING: {null_rows} patients were unable to have the score calculated"
            f" as their input parameters were invalid or out of range."
        )
