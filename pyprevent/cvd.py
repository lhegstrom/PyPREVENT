import pandas as pd
import math
from pyprevent import _pyprevent

"""
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
    return _pyprevent.calculate_10_yr_ascvd_rust(
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
    return _pyprevent.calculate_30_yr_ascvd_rust(
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
"""


def calculate_10_yr_cvd_risk(
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
    """
    Calculate the 10-year risk of cardiovascular disease .

    This function uses various health and lifestyle parameters to estimate the risk
    of developing CVD over the next 10 years. The input values must fall within
    specified ranges for the calculation to be valid.

    Parameters:
    - sex (str): The sex of the individual ('male' or 'female'). Case insensitive.
    - age (float): Age of the individual in years. Must be between 30 and 79.
    - total_cholesterol (float): Total cholesterol level (mg/dL). Must be between 130 and 320.
    - hdl_cholesterol (float): High-density lipoprotein cholesterol level (mg/dL). Must be between 20 and 100.
    - systolic_bp (float): Systolic blood pressure (mmHg). Must be between 90 and 200.
    - has_diabetes (bool): Indicates if the individual has diabetes (True or False).
    - current_smoker (bool): Indicates if the individual is a current smoker (True or False).
    - bmi (float): Body mass index (kg/m^2). Must be between 18.5 and 39.9.
    - egfr (float): Estimated glomerular filtration rate (mL/min/1.73 m^2). Must be between 15 and 140.
    - on_htn_meds (bool): Indicates if the individual is on hypertension medication (True or False).
    - on_cholesterol_meds (bool): Indicates if the individual is on cholesterol-lowering medication (True or False).

    Returns:
    float: The estimated 10-year CVD risk percentage.

    Raises:
    ValueError: If any of the input parameters are outside their valid ranges.

    Example:
    >>> calculate_10_yr_cvd_risk(
    ...     sex="male",
    ...     age=45,
    ...     total_cholesterol=210,
    ...     hdl_cholesterol=55,
    ...     systolic_bp=130,
    ...     has_diabetes=False,
    ...     current_smoker=True,
    ...     bmi=28,
    ...     egfr=65,
    ...     on_htn_meds=False,
    ...     on_cholesterol_meds=False,
    ... )
    # Returns: Estimated 10-year CVD risk percentage (e.g., 12.3)
    """
    return _pyprevent.calculate_10_yr_cvd_rust(
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


def calculate_30_yr_cvd_risk(
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
    """
    Calculate the 30-year risk of cardiovascular disease (CVD).

    This function uses various health and lifestyle parameters to estimate the risk
    of developing CVD over the next 30 years. The input values must fall within
    specified ranges for the calculation to be valid.

    Parameters:
    - sex (str): The sex of the individual ('male' or 'female'). Case insensitive.
    - age (float): Age of the individual in years. Must be between 30 and 59.
    - total_cholesterol (float): Total cholesterol level (mg/dL). Must be between 130 and 320.
    - hdl_cholesterol (float): High-density lipoprotein cholesterol level (mg/dL). Must be between 20 and 100.
    - systolic_bp (float): Systolic blood pressure (mmHg). Must be between 90 and 200.
    - has_diabetes (bool): Indicates if the individual has diabetes (True or False).
    - current_smoker (bool): Indicates if the individual is a current smoker (True or False).
    - bmi (float): Body mass index (kg/m^2). Must be between 18.5 and 39.9.
    - egfr (float): Estimated glomerular filtration rate (mL/min/1.73 m^2). Must be between 15 and 140.
    - on_htn_meds (bool): Indicates if the individual is on hypertension medication (True or False).
    - on_cholesterol_meds (bool): Indicates if the individual is on cholesterol-lowering medication (True or False).

    Returns:
    float: The estimated 30-year CVD risk percentage.

    Raises:
    ValueError: If any of the input parameters are outside their valid ranges.

    Example:
    >>> calculate_30_yr_cvd_risk(
    ...     sex="male",
    ...     age=45,
    ...     total_cholesterol=210,
    ...     hdl_cholesterol=55,
    ...     systolic_bp=130,
    ...     has_diabetes=False,
    ...     current_smoker=True,
    ...     bmi=28,
    ...     egfr=65,
    ...     on_htn_meds=False,
    ...     on_cholesterol_meds=False,
    ... )
    # Returns: Estimated 30-year CVD risk percentage (e.g., 12.3)
    """
    return _pyprevent.calculate_30_yr_cvd_rust(
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


def batch_calculate_10_yr_cvd_risk(
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
) -> list:
    """
    Batch calculate the 10-year risk of cardiovascular disease (CVD) for a dataset.

    This function processes a DataFrame with health and lifestyle parameters to estimate
    the 10-year CVD risk for each individual in the dataset. The input values must fall
    within specified ranges for the calculation to be valid.

    Parameters:
    - df (pd.DataFrame): DataFrame containing the required data.
    - sex (str): Column name for the sex of the individuals ('male' or 'female'). Case insensitive. Default 'sex'.
    - age (str): Column name for the age of the individuals in years. Must be between 30 and 79. Default 'age'.
    - total_cholesterol (str): Column name for the total cholesterol level (mg/dL). Must be between 130 and 320. Default 'total_cholesterol'.
    - hdl_cholesterol (str): Column name for the HDL cholesterol level (mg/dL). Must be between 20 and 100. Default 'hdl_cholesterol'.
    - systolic_bp (str): Column name for the systolic blood pressure (mmHg). Must be between 90 and 200. Default 'systolic_bp'.
    - has_diabetes (str): Column name indicating if the individual has diabetes (True or False). Default 'has_diabetes'.
    - current_smoker (str): Column name indicating if the individual is a current smoker (True or False). Default 'current_smoker'.
    - bmi (str): Column name for the body mass index (kg/m^2). Must be between 18.5 and 39.9. Default 'bmi'.
    - egfr (str): Column name for the estimated glomerular filtration rate (mL/min/1.73 m^2). Must be between 15 and 140. Default 'egfr'.
    - on_htn_meds (str): Column name indicating if the individual is on hypertension medication (True or False). Default 'on_htn_meds'.
    - on_cholesterol_meds (str): Column name indicating if the individual is on cholesterol-lowering medication (True or False). Default 'on_cholesterol_meds'.
    - **kwargs: Optional keyword arguments for custom column mappings.

    Returns:
    list: A list of estimated 10-year CVD risk percentages for each individual in the DataFrame.

    Raises:
    ValueError: If any of the input parameters are outside their valid ranges or if the specified columns are not found in the DataFrame.

    Example:
    >>> df = pd.DataFrame({...})
    >>> risks = batch_calculate_10_yr_cvd_risk(df, sex='gender', age='patient_age', ...)
    >>> print(risks)
    # Returns: A list of estimated 10-year CVD risk percentages

    Alternatively, can pass in a dictionary of column mappings
    >>> df = pd.DataFrame({...})
    >>> column_mappings = {
    ...     'sex': 'gender_column',
    ...     'age': 'age_column',
    ...     'total_cholesterol': 'cholesterol_total',
    ...     'hdl_cholesterol': 'cholesterol_hdl',
    ...     'systolic_bp': 'bp_systolic',
    ...     'has_diabetes': 'diabetes_status',
    ...     'current_smoker': 'smoker_status',
    ...     'bmi': 'body_mass_index',
    ...     'egfr': 'glomerular_rate',
    ...     'on_htn_meds': 'hypertension_meds',
    ...     'on_cholesterol_meds': 'cholesterol_meds'
    ... }
    >>> risks = batch_calculate_10_yr_cvd_risk(df, **column_mappings)
    # Returns: A list of estimated 10-year CVD risk percentages
    """

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

    # Reorder DataFrame columns based on the column mapping
    reordered_df = df[[column_mapping[key] for key in column_mapping]].copy()

    # Calculate ASCVD risk for each row
    return [
        _pyprevent.calculate_10_yr_cvd_rust(*row)
        for row in reordered_df.itertuples(index=False)
    ]


def batch_calculate_30_yr_cvd_risk(
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
) -> list:
    """
    Batch calculate the 30-year risk of cardiovascular disease (CVD) for a dataset.

    This function processes a DataFrame with health and lifestyle parameters to estimate
    the 30-year CVD risk for each individual in the dataset. The input values must fall
    within specified ranges for the calculation to be valid.

    Parameters:
    - df (pd.DataFrame): DataFrame containing the required data.
    - sex (str): Column name for the sex of the individuals ('male' or 'female'). Case insensitive. Default 'sex'.
    - age (str): Column name for the age of the individuals in years. Must be between 30 and 59. Default 'age'.
    - total_cholesterol (str): Column name for the total cholesterol level (mg/dL). Must be between 130 and 320. Default 'total_cholesterol'.
    - hdl_cholesterol (str): Column name for the HDL cholesterol level (mg/dL). Must be between 20 and 100. Default 'hdl_cholesterol'.
    - systolic_bp (str): Column name for the systolic blood pressure (mmHg). Must be between 90 and 200. Default 'systolic_bp'.
    - has_diabetes (str): Column name indicating if the individual has diabetes (True or False). Default 'has_diabetes'.
    - current_smoker (str): Column name indicating if the individual is a current smoker (True or False). Default 'current_smoker'.
    - bmi (str): Column name for the body mass index (kg/m^2). Must be between 18.5 and 39.9. Default 'bmi'.
    - egfr (str): Column name for the estimated glomerular filtration rate (mL/min/1.73 m^2). Must be between 15 and 140. Default 'egfr'.
    - on_htn_meds (str): Column name indicating if the individual is on hypertension medication (True or False). Default 'on_htn_meds'.
    - on_cholesterol_meds (str): Column name indicating if the individual is on cholesterol-lowering medication (True or False). Default 'on_cholesterol_meds'.
    - **kwargs: Optional keyword arguments for custom column mappings.

    Returns:
    list: A list of estimated 30-year CVD risk percentages for each individual in the DataFrame.

    Raises:
    ValueError: If any of the input parameters are outside their valid ranges or if the specified columns are not found in the DataFrame.

    Example:
    >>> df = pd.DataFrame({...})
    >>> risks = batch_calculate_30_yr_heart_failure_risk(df, sex='gender', age='patient_age', ...)
    >>> print(risks)
    # Returns: A list of estimated 30-year CVD risk percentages

    Alternatively, can pass in a dictionary of column mappings
    >>> df = pd.DataFrame({...})
    >>> column_mappings = {
    ...     'sex': 'gender_column',
    ...     'age': 'age_column',
    ...     'total_cholesterol': 'cholesterol_total',
    ...     'hdl_cholesterol': 'cholesterol_hdl',
    ...     'systolic_bp': 'bp_systolic',
    ...     'has_diabetes': 'diabetes_status',
    ...     'current_smoker': 'smoker_status',
    ...     'bmi': 'body_mass_index',
    ...     'egfr': 'glomerular_rate',
    ...     'on_htn_meds': 'hypertension_meds',
    ...     'on_cholesterol_meds': 'cholesterol_meds'
    ... }
    >>> risks = batch_calculate_30_yr_heart_failure_risk(df, **column_mappings)
    # Returns: A list of estimated 30-year CVD risk percentages
    """
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

    # Reorder DataFrame columns based on the column mapping
    reordered_df = df[[column_mapping[key] for key in column_mapping]].copy()

    # Calculate CVD risk for each row
    return [
        _pyprevent.calculate_30_yr_cvd_rust(*row)
        for row in reordered_df.itertuples(index=False)
    ]
