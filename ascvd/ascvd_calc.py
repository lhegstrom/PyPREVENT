import pandas as pd


def calculate_10_yr_ascvd_risk(
    sex: str,
    age: float,
    total_cholesterol: float,
    hdl_cholesterol: float,
    systolic_bp: float,
    has_diabetes: bool,
    current_smoker: bool,
    bmi: float,
    egfr: float,
    on_htn_meds: bool,
    on_cholesterol_meds: bool,
) -> float:
    from ascvd import _ascvd
    return _ascvd.calculate_10_yr_ascvd_rust(
        sex,
        age,
        total_cholesterol,
        hdl_cholesterol,
        systolic_bp,
        has_diabetes,
        current_smoker,
        bmi,
        egfr,
        on_htn_meds,
        on_cholesterol_meds,
    )


def calculate_30_yr_ascvd_risk(
    sex: str,
    age: float,
    total_cholesterol: float,
    hdl_cholesterol: float,
    systolic_bp: float,
    has_diabetes: bool,
    current_smoker: bool,
    bmi: float,
    egfr: float,
    on_htn_meds: bool,
    on_cholesterol_meds: bool,
) -> float:
    from ascvd import _ascvd
    return _ascvd.calculate_30_yr_ascvd_rust(
        sex,
        age,
        total_cholesterol,
        hdl_cholesterol,
        systolic_bp,
        has_diabetes,
        current_smoker,
        bmi,
        egfr,
        on_htn_meds,
        on_cholesterol_meds,
    )


def batch_calculate_10_yr_ascvd_risk(df: pd.DataFrame) -> list:
    from ascvd import _ascvd
    new_df = df.copy()
    return [
        _ascvd.calculate_10_yr_ascvd_rust(*row)
        for row in new_df.itertuples(index=False)
    ]


def batch_calculate_30_yr_ascvd_risk(df: pd.DataFrame) -> list:
    from ascvd import _ascvd
    new_df = df.copy()
    return [
        _ascvd.calculate_30_yr_ascvd_rust(*row)
        for row in new_df.itertuples(index=False)
    ]
