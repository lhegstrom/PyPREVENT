from concurrent.futures import ThreadPoolExecutor

import numpy as np
import pandas as pd
import pytest
from hypothesis import given, settings

import pyprevent
from pyprevent import _pyprevent
from pyprevent.utils import _prepare_df_for_batch

from .fixtures import generate_30_yr_test_case

MODELS = [
    (years, model) for years in (10, 30) for model in ("ascvd", "cvd", "heart_failure")
]
COLUMNS = [
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
]
PATIENT = ["female", 40, 200, 50, 120, True, False, 25, 70, True, False]


def functions(years, model):
    scalar = getattr(pyprevent, f"calculate_{years}_yr_{model}_risk")
    batch = getattr(pyprevent, f"batch_calculate_{years}_yr_{model}_risk")
    native_model = "hf" if model == "heart_failure" else model
    native = getattr(
        _pyprevent, f"calculate_{years}_yr_{native_model}_rust_parallel_np"
    )
    return scalar, batch, native


@pytest.mark.parametrize("years,model", MODELS)
@pytest.mark.parametrize(
    "column,bad",
    [("sex", value) for value in ("unknown", None, np.nan, 1)]
    + [
        (key, value)
        for key in COLUMNS
        if key
        in ("has_diabetes", "current_smoker", "on_htn_meds", "on_cholesterol_meds")
        for value in (2, -1, 0.5, np.nan, None, "True")
    ],
)
def test_invalid_categories_do_not_produce_scores(years, model, column, bad, capsys):
    scalar, batch, _ = functions(years, model)
    frame = pd.DataFrame([PATIENT, PATIENT], columns=COLUMNS).astype(object)
    frame.loc[0, column] = bad
    result = batch(frame)
    assert np.isnan(result[0])
    assert result[1] == pytest.approx(scalar(*PATIENT), rel=1e-13)
    assert "WARNING: 1 patients" in capsys.readouterr().out


@pytest.mark.parametrize("years,model", MODELS)
def test_empty_and_strided_native_arrays(years, model):
    scalar, batch, native = functions(years, model)
    frame = pd.DataFrame([PATIENT] * 6, columns=COLUMNS)
    frame["age"] = np.arange(40, 46)
    frame["sex"] = ["female", "male"] * 3
    assert batch(frame.iloc[:0]).shape == (0,)
    array = _prepare_df_for_batch(frame)
    expected = np.array(
        [scalar(*row) for row in frame.itertuples(index=False, name=None)]
    )
    backing = np.empty((6, 22))
    backing[:, ::2] = array
    for view, scores in (
        (array[::2], expected[::2]),
        (np.asfortranarray(array), expected),
        (array[::-1], expected[::-1]),
        (backing[:, ::2], expected),
    ):
        np.testing.assert_allclose(native(view), scores, rtol=1e-13)
    with pytest.raises(ValueError, match="shape"):
        native(array[:, :10])
    with pytest.raises(ValueError, match="shape"):
        native(array[0])


@pytest.mark.parametrize("years,model", MODELS)
@pytest.mark.parametrize("column", [0, 5, 6, 9, 10])
def test_native_categories_are_validated(years, model, column):
    _, _, native = functions(years, model)
    array = _prepare_df_for_batch(pd.DataFrame([PATIENT] * 5, columns=COLUMNS))
    array[:, column] = [np.nan, np.inf, -1, 2, 0.5]
    assert np.isnan(native(array)).all()


@pytest.mark.parametrize("years,model", MODELS)
@pytest.mark.parametrize(
    "column",
    ["age", "total_cholesterol", "hdl_cholesterol", "systolic_bp", "bmi", "egfr"],
)
def test_invalid_numeric_rows(years, model, column):
    _, batch, _ = functions(years, model)
    frame = pd.DataFrame([PATIENT] * 5, columns=COLUMNS).astype(object)
    frame[column] = [np.nan, np.inf, -np.inf, "invalid", -1]
    assert np.isnan(batch(frame)).all()


@pytest.mark.parametrize("years,model", MODELS)
@settings(max_examples=40, deadline=None)
@given(patient=generate_30_yr_test_case())
def test_scalar_batch_parity(years, model, patient):
    scalar, batch, _ = functions(years, model)
    assert batch(pd.DataFrame([patient]))[0] == pytest.approx(
        scalar(**patient), rel=1e-13, abs=1e-13
    )


def test_nullable_columns_and_custom_names_preserve_input():
    frame = pd.DataFrame([PATIENT, PATIENT], columns=COLUMNS).rename(
        columns={"sex": "gender"}
    )
    frame["has_diabetes"] = pd.Series([True, pd.NA], dtype="boolean")
    original = frame.copy(deep=True)
    result = pyprevent.batch_calculate_10_yr_cvd_risk(frame, sex="gender")
    assert np.isfinite(result[0]) and np.isnan(result[1])
    pd.testing.assert_frame_equal(frame, original)


def test_column_errors():
    frame = pd.DataFrame([PATIENT], columns=COLUMNS)
    with pytest.raises(ValueError, match="not found"):
        _prepare_df_for_batch(frame.drop(columns="age"))
    with pytest.raises(TypeError, match="Unexpected"):
        _prepare_df_for_batch(frame, typo="age")
    with pytest.raises(ValueError, match="unique"):
        _prepare_df_for_batch(pd.concat([frame, frame[["age"]]], axis=1))


def test_concurrent_native_calls():
    _, _, native = functions(10, "cvd")
    array = _prepare_df_for_batch(pd.DataFrame([PATIENT] * 10000, columns=COLUMNS))
    expected = native(array)
    with ThreadPoolExecutor(max_workers=2) as pool:
        for result in pool.map(native, [array, array]):
            np.testing.assert_array_equal(result, expected)
