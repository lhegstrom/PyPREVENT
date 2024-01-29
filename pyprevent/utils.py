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

    # Update column names with mappings from kwargs if provided
    column_mapping.update(kwargs)

    # Ensure all required column names exist in the DataFrame
    for key, col in column_mapping.items():
        if col not in df.columns:
            raise ValueError(
                f"Column '{col}' for parameter '{key}' not found in DataFrame."
            )

    reordered_df = df[[column_mapping[key] for key in column_mapping]].copy()

    sex_col = column_mapping["sex"]
    reordered_df[sex_col] = (
        reordered_df[sex_col].str.lower().map({"female": 0, "male": 1})
    )

    # Convert boolean columns to integers based on the column names from column_mapping
    bool_cols = [
        column_mapping["has_diabetes"],
        column_mapping["current_smoker"],
        column_mapping["on_htn_meds"],
        column_mapping["on_cholesterol_meds"],
    ]
    reordered_df[bool_cols] = reordered_df[bool_cols].astype(int)
    return reordered_df.values.astype(np.float64)


def _report_any_null_values(result: np.ndarray) -> None:
    null_rows = np.isnan(result).sum()
    if null_rows > 0:
        print(
            f"WARNING: {null_rows} patients were unable to have the score calculated"
            f"as their input parameters were out of the range."
        )
