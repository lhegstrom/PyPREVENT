from pyprevent import _pyprevent
import pandas as pd


def calculate_10_yr_heart_failure(
    sex: str,
    age: float,
    total_cholesterol: float,
    hdl_cholesterol: float,
    systolic_bp: float,
    has_diabetes: bool,
    is_smoker: bool,
    bmi: float,
    egfr: float,
    on_meds: bool,
    cholesterol_treated: bool,
) -> float:
    return _pyprevent.calculate_10_yr_heart_failure_rust(
        sex,
        age,
        total_cholesterol,
        hdl_cholesterol,
        systolic_bp,
        has_diabetes,
        is_smoker,
        bmi,
        egfr,
        on_meds,
        cholesterol_treated,
    )


def calculate_30_yr_heart_failure(
    sex: str,
    age: float,
    total_cholesterol: float,
    hdl_cholesterol: float,
    systolic_bp: float,
    has_diabetes: bool,
    is_smoker: bool,
    bmi: float,
    egfr: float,
    on_meds: bool,
    cholesterol_treated: bool,
) -> float:
    return _pyprevent.calculate_30_yr_heart_failure_rust(
        sex,
        age,
        total_cholesterol,
        hdl_cholesterol,
        systolic_bp,
        has_diabetes,
        is_smoker,
        bmi,
        egfr,
        on_meds,
        cholesterol_treated,
    )


def batch_calculate_10_yr_heart_failure_risk(df: pd.DataFrame) -> list:
    new_df = df.copy()
    return [
        _pyprevent.calculate_10_yr_heart_failure_rust(*row)
        for row in new_df.itertuples(index=False)
    ]


def batch_calculate_30_yr_heart_failure_risk(df: pd.DataFrame) -> list:
    new_df = df.copy()
    return [
        _pyprevent.calculate_30_yr_heart_failure_rust(*row)
        for row in new_df.itertuples(index=False)
    ]
