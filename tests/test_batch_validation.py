from collections.abc import Callable
from concurrent.futures import ThreadPoolExecutor
from typing import Any

import numpy as np
import pandas as pd
import pytest
from hypothesis import given, settings

import pyprevent
from pyprevent import _pyprevent
from pyprevent.batch_preparation import _build_native_batch_array

from .fixtures import generate_30_yr_test_case

YEAR_AND_MODEL_PAIRS = [
    (years, model) for years in (10, 30) for model in ("ascvd", "cvd", "heart_failure")
]
BOOLEAN_COLUMN_NAMES = [
    "has_diabetes",
    "current_smoker",
    "on_htn_meds",
    "on_cholesterol_meds",
]
NATIVE_FLAG_COLUMN_INDEXES = [0, 5, 6, 9, 10]
VALID_PATIENT: dict[str, Any] = {
    "sex": "female",
    "age": 40,
    "total_cholesterol": 200,
    "hdl_cholesterol": 50,
    "systolic_bp": 120,
    "has_diabetes": True,
    "current_smoker": False,
    "bmi": 25,
    "egfr": 70,
    "on_htn_meds": True,
    "on_cholesterol_meds": False,
}

RiskFunctions = tuple[
    Callable[..., float], Callable[..., np.ndarray], Callable[..., np.ndarray]
]


def load_risk_functions(years: int, model: str) -> RiskFunctions:
    """Return the public scalar, public batch, and native batch functions for one model."""
    scalar_function = getattr(pyprevent, f"calculate_{years}_yr_{model}_risk")
    batch_function = getattr(pyprevent, f"batch_calculate_{years}_yr_{model}_risk")
    native_model = "hf" if model == "heart_failure" else model
    native_function = getattr(
        _pyprevent, f"calculate_{years}_yr_{native_model}_rust_parallel_np"
    )
    return scalar_function, batch_function, native_function


def build_patient_frame(row_count: int) -> pd.DataFrame:
    return pd.DataFrame([VALID_PATIENT] * row_count)


@pytest.mark.parametrize("years,model", YEAR_AND_MODEL_PAIRS)
@pytest.mark.parametrize(
    "column_name,invalid_value",
    [("sex", value) for value in ("unknown", None, np.nan, 1)]
    + [
        (column_name, value)
        for column_name in BOOLEAN_COLUMN_NAMES
        for value in (2, -1, 0.5, np.nan, None, "True")
    ],
)
def test_invalid_categories_do_not_produce_scores(
    years: int,
    model: str,
    column_name: str,
    invalid_value: Any,
    capsys: pytest.CaptureFixture[str],
) -> None:
    scalar_function, batch_function, _ = load_risk_functions(years=years, model=model)
    patient_frame = build_patient_frame(row_count=2).astype(object)
    patient_frame.loc[0, column_name] = invalid_value
    risk_percentages = batch_function(df=patient_frame)
    assert np.isnan(risk_percentages[0])
    assert risk_percentages[1] == pytest.approx(
        scalar_function(**VALID_PATIENT), rel=1e-13
    )
    assert "WARNING: 1 patients" in capsys.readouterr().out


@pytest.mark.parametrize("years,model", YEAR_AND_MODEL_PAIRS)
def test_empty_and_strided_native_arrays(years: int, model: str) -> None:
    scalar_function, batch_function, native_function = load_risk_functions(
        years=years, model=model
    )
    patients = [
        {**VALID_PATIENT, "age": age, "sex": sex}
        for age, sex in zip(range(40, 46), ["female", "male"] * 3)
    ]
    patient_frame = pd.DataFrame(patients)
    assert batch_function(df=patient_frame.iloc[:0]).shape == (0,)
    native_array = _build_native_batch_array(df=patient_frame)
    expected_risks = np.array([scalar_function(**patient) for patient in patients])
    backing_array = np.empty((6, 22))
    backing_array[:, ::2] = native_array
    for array_view, view_expected_risks in (
        (native_array[::2], expected_risks[::2]),
        (np.asfortranarray(native_array), expected_risks),
        (native_array[::-1], expected_risks[::-1]),
        (backing_array[:, ::2], expected_risks),
    ):
        np.testing.assert_allclose(
            native_function(data=array_view), view_expected_risks, rtol=1e-13
        )
    with pytest.raises(ValueError, match="shape"):
        native_function(data=native_array[:, :10])
    with pytest.raises(ValueError, match="shape"):
        native_function(data=native_array[0])


@pytest.mark.parametrize("years,model", YEAR_AND_MODEL_PAIRS)
@pytest.mark.parametrize("column_index", NATIVE_FLAG_COLUMN_INDEXES)
def test_native_categories_are_validated(
    years: int, model: str, column_index: int
) -> None:
    _, _, native_function = load_risk_functions(years=years, model=model)
    native_array = _build_native_batch_array(df=build_patient_frame(row_count=5))
    native_array[:, column_index] = [np.nan, np.inf, -1, 2, 0.5]
    assert np.isnan(native_function(data=native_array)).all()


@pytest.mark.parametrize("years,model", YEAR_AND_MODEL_PAIRS)
@pytest.mark.parametrize(
    "column_name",
    ["age", "total_cholesterol", "hdl_cholesterol", "systolic_bp", "bmi", "egfr"],
)
def test_invalid_numeric_rows(years: int, model: str, column_name: str) -> None:
    _, batch_function, _ = load_risk_functions(years=years, model=model)
    patient_frame = build_patient_frame(row_count=5).astype(object)
    patient_frame[column_name] = [np.nan, np.inf, -np.inf, "invalid", -1]
    assert np.isnan(batch_function(df=patient_frame)).all()


@pytest.mark.parametrize("years,model", YEAR_AND_MODEL_PAIRS)
@settings(max_examples=40, deadline=None)
@given(patient=generate_30_yr_test_case())
def test_scalar_batch_parity(years: int, model: str, patient: dict[str, Any]) -> None:
    scalar_function, batch_function, _ = load_risk_functions(years=years, model=model)
    assert batch_function(df=pd.DataFrame([patient]))[0] == pytest.approx(
        scalar_function(**patient), rel=1e-13, abs=1e-13
    )


def test_nullable_columns_and_custom_names_preserve_input() -> None:
    patient_frame = build_patient_frame(row_count=2).rename(columns={"sex": "gender"})
    patient_frame["has_diabetes"] = pd.Series([True, pd.NA], dtype="boolean")
    original_frame = patient_frame.copy(deep=True)
    risk_percentages = pyprevent.batch_calculate_10_yr_cvd_risk(
        df=patient_frame, sex="gender"
    )
    assert np.isfinite(risk_percentages[0]) and np.isnan(risk_percentages[1])
    pd.testing.assert_frame_equal(patient_frame, original_frame)


def test_column_errors() -> None:
    patient_frame = build_patient_frame(row_count=1)
    with pytest.raises(ValueError, match="not found"):
        _build_native_batch_array(df=patient_frame.drop(columns="age"))
    with pytest.raises(TypeError, match="Unexpected"):
        _build_native_batch_array(df=patient_frame, typo="age")
    with pytest.raises(ValueError, match="unique"):
        _build_native_batch_array(
            df=pd.concat([patient_frame, patient_frame[["age"]]], axis=1)
        )


def test_concurrent_native_calls() -> None:
    _, _, native_function = load_risk_functions(years=10, model="cvd")
    native_array = _build_native_batch_array(df=build_patient_frame(row_count=10000))
    expected_risks = native_function(data=native_array)
    with ThreadPoolExecutor(max_workers=2) as pool:
        for risk_percentages in pool.map(native_function, [native_array, native_array]):
            np.testing.assert_array_equal(risk_percentages, expected_risks)
