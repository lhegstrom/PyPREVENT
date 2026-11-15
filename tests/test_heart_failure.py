from typing import Any

import numpy as np
import pandas as pd
import pytest
from hypothesis import given

import pyprevent

from .fixtures import (
    TEST_PATIENT,
    TEST_PATIENT_INPUTS,
    build_patient,
    generate_10_yr_test_case,
    generate_30_yr_test_case,
)


def test_calculate_10_yr_heart_failure_risk_basic() -> None:
    result = pyprevent.calculate_10_yr_heart_failure_risk(**build_patient())
    # Baseline at 89e99bb includes the diabetes coefficient correction in d0185be.
    assert result == pytest.approx(4.3114057073104215, rel=1e-13)

    result = pyprevent.calculate_10_yr_heart_failure_risk(
        **build_patient(
            sex="MALE",
            age=68,
            total_cholesterol=300,
            hdl_cholesterol=85,
            systolic_bp=150,
            has_diabetes=False,
            bmi=35,
            egfr=65,
            on_htn_meds=False,
        )
    )
    assert result == pytest.approx(13.1, abs=0.1)


def test_calculate_10_yr_hf_risk_default_patient() -> None:
    result = pyprevent.calculate_10_yr_heart_failure_risk(**TEST_PATIENT_INPUTS)
    expected_result = TEST_PATIENT["10_yr_hf_expected"]
    assert result == pytest.approx(expected_result, abs=0.1)


def test_calculate_30_yr_hf_risk_default_patient() -> None:
    result = pyprevent.calculate_30_yr_heart_failure_risk(**TEST_PATIENT_INPUTS)
    expected_result = TEST_PATIENT["30_yr_hf_expected"]
    assert result == pytest.approx(expected_result, abs=0.1)


@given(generate_10_yr_test_case())
def test_calculate_10_yr_hf_risk_with_hypothesis(test_case: dict[str, Any]) -> None:
    result = pyprevent.calculate_10_yr_heart_failure_risk(**test_case)
    assert np.isfinite(result)
    assert 0 <= result <= 100


@given(generate_30_yr_test_case())
def test_calculate_30_yr_hf_risk_with_hypothesis(test_case: dict[str, Any]) -> None:
    result = pyprevent.calculate_30_yr_heart_failure_risk(**test_case)
    assert np.isfinite(result)
    assert 0 <= result <= 100


def test_invalid_age() -> None:
    with pytest.raises(ValueError):
        pyprevent.calculate_10_yr_heart_failure_risk(**build_patient(age=29))


def test_invalid_total_cholesterol() -> None:
    with pytest.raises(ValueError):
        pyprevent.calculate_10_yr_heart_failure_risk(
            **build_patient(total_cholesterol=129)
        )


def test_invalid_hdl_cholesterol() -> None:
    with pytest.raises(ValueError):
        pyprevent.calculate_10_yr_heart_failure_risk(
            **build_patient(hdl_cholesterol=19)
        )


def test_invalid_systolic_bp() -> None:
    with pytest.raises(ValueError):
        pyprevent.calculate_10_yr_heart_failure_risk(**build_patient(systolic_bp=89))


def test_invalid_bmi() -> None:
    with pytest.raises(ValueError):
        pyprevent.calculate_10_yr_heart_failure_risk(**build_patient(bmi=18.4))


def test_invalid_egfr() -> None:
    with pytest.raises(ValueError):
        pyprevent.calculate_10_yr_heart_failure_risk(**build_patient(egfr=14.9))


def test_invalid_sex() -> None:
    with pytest.raises(ValueError):
        pyprevent.calculate_10_yr_heart_failure_risk(**build_patient(sex="unknown"))


def test_calculate_30_yr_heart_failure_risk_basic() -> None:
    result = pyprevent.calculate_30_yr_heart_failure_risk(**build_patient())
    assert result == pytest.approx(26.0, abs=0.1)

    result = pyprevent.calculate_30_yr_heart_failure_risk(**build_patient(sex="MALE"))
    assert result == pytest.approx(23.5, abs=0.1)


def test_invalid_age_30() -> None:
    with pytest.raises(ValueError):
        pyprevent.calculate_30_yr_heart_failure_risk(**build_patient(age=29))


def test_invalid_total_cholesterol_30() -> None:
    with pytest.raises(ValueError):
        pyprevent.calculate_30_yr_heart_failure_risk(
            **build_patient(total_cholesterol=129)
        )


def test_invalid_hdl_cholesterol_30() -> None:
    with pytest.raises(ValueError):
        pyprevent.calculate_30_yr_heart_failure_risk(
            **build_patient(hdl_cholesterol=19)
        )


def test_invalid_systolic_bp_30() -> None:
    with pytest.raises(ValueError):
        pyprevent.calculate_30_yr_heart_failure_risk(**build_patient(systolic_bp=89))


def test_invalid_bmi_30() -> None:
    with pytest.raises(ValueError):
        pyprevent.calculate_30_yr_heart_failure_risk(**build_patient(bmi=18.4))


def test_invalid_egfr_30() -> None:
    with pytest.raises(ValueError):
        pyprevent.calculate_30_yr_heart_failure_risk(**build_patient(egfr=14.9))


def test_invalid_sex_30() -> None:
    with pytest.raises(ValueError):
        pyprevent.calculate_30_yr_heart_failure_risk(**build_patient(sex="unknown"))


def test_batch_calculate_10_yr_heart_failure_risk() -> None:
    df = pd.DataFrame([build_patient()] * 10)

    result = pyprevent.batch_calculate_10_yr_heart_failure_risk(df=df)

    # Regression value after the upstream d0185be coefficient correction.
    expected_result = [4.3114057073104215] * 10

    np.testing.assert_allclose(result, expected_result, rtol=1e-13, atol=1e-13)


def test_batch_calculate_30_yr_heart_failure_risk() -> None:
    df = pd.DataFrame([build_patient()] * 10)

    result = pyprevent.batch_calculate_30_yr_heart_failure_risk(df=df)

    expected_result = [26.005234843826347] * 10

    np.testing.assert_allclose(result, expected_result, rtol=1e-13, atol=1e-13)
