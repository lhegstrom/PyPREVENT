"""Type stub for the native module built from src/ by Maturin."""

from typing import TypedDict

import numpy as np
import numpy.typing as npt

class RiskFactors(TypedDict):
    """Keys must match the fields of `RiskFactors` in src/risk_factors.rs."""

    age: float
    total_cholesterol: float
    hdl_cholesterol: float
    systolic_bp: float
    has_diabetes: bool
    current_smoker: bool
    bmi: float
    egfr: float
    on_htn_meds: bool
    on_cholesterol_meds: bool

def calculate_10_yr_ascvd_rust(sex: str, risk_factors: RiskFactors) -> float: ...
def calculate_30_yr_ascvd_rust(sex: str, risk_factors: RiskFactors) -> float: ...
def calculate_10_yr_cvd_rust(sex: str, risk_factors: RiskFactors) -> float: ...
def calculate_30_yr_cvd_rust(sex: str, risk_factors: RiskFactors) -> float: ...
def calculate_10_yr_heart_failure_rust(
    sex: str, risk_factors: RiskFactors
) -> float: ...
def calculate_30_yr_heart_failure_rust(
    sex: str, risk_factors: RiskFactors
) -> float: ...
def calculate_10_yr_ascvd_rust_parallel_np(
    data: npt.NDArray[np.float64],
) -> npt.NDArray[np.float64]: ...
def calculate_30_yr_ascvd_rust_parallel_np(
    data: npt.NDArray[np.float64],
) -> npt.NDArray[np.float64]: ...
def calculate_10_yr_cvd_rust_parallel_np(
    data: npt.NDArray[np.float64],
) -> npt.NDArray[np.float64]: ...
def calculate_30_yr_cvd_rust_parallel_np(
    data: npt.NDArray[np.float64],
) -> npt.NDArray[np.float64]: ...
def calculate_10_yr_hf_rust_parallel_np(
    data: npt.NDArray[np.float64],
) -> npt.NDArray[np.float64]: ...
def calculate_30_yr_hf_rust_parallel_np(
    data: npt.NDArray[np.float64],
) -> npt.NDArray[np.float64]: ...
