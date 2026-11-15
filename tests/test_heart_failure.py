import numpy as np
import pandas as pd
import pytest
from hypothesis import given

import pyprevent

from .fixtures import TEST_PATIENT, generate_10_yr_test_case, generate_30_yr_test_case


def test_calculate_10_yr_heart_failure_basic():
    # Test with some basic input values
    result = pyprevent.calculate_10_yr_heart_failure_risk(
        "female", 40, 200, 50, 120, True, True, 25, 70, True, True
    )
    # Check if the result is as expected
    # Baseline at 89e99bb includes the diabetes coefficient correction in d0185be.
    assert result == pytest.approx(4.3114057073104215, rel=1e-13)

    result = pyprevent.calculate_10_yr_heart_failure_risk(
        "MALE", 68, 300, 85, 150, False, True, 35, 65, False, True
    )
    # Check if the result is as expected
    assert result == pytest.approx(13.1, abs=0.1)


def test_calculate_10_yr_hf_risk_default_patient():
    test_dict = {k: v for k, v in TEST_PATIENT.items() if not k.endswith("_expected")}
    result = pyprevent.calculate_10_yr_heart_failure_risk(**test_dict)
    expected_result = TEST_PATIENT["10_yr_hf_expected"]
    assert result == pytest.approx(expected_result, abs=0.1)


def test_calculate_30_yr_hf_risk_default_patient():
    test_dict = {k: v for k, v in TEST_PATIENT.items() if not k.endswith("_expected")}
    result = pyprevent.calculate_30_yr_heart_failure_risk(**test_dict)
    expected_result = TEST_PATIENT["30_yr_hf_expected"]
    assert result == pytest.approx(expected_result, abs=0.1)


@given(generate_10_yr_test_case())
def test_calculate_10_yr_hf_risk_with_hypothesis(test_case):
    result = pyprevent.calculate_10_yr_heart_failure_risk(**test_case)
    assert np.isfinite(result)
    assert 0 <= result <= 100


@given(generate_30_yr_test_case())
def test_calculate_30_yr_hf_risk_with_hypothesis(test_case):
    result = pyprevent.calculate_30_yr_heart_failure_risk(**test_case)
    assert np.isfinite(result)
    assert 0 <= result <= 100


def test_invalid_age():
    with pytest.raises(ValueError):
        pyprevent.calculate_10_yr_heart_failure_risk(
            "female", 29, 200, 50, 120, True, True, 25, 70, True, True
        )


def test_invalid_total_cholesterol():
    with pytest.raises(ValueError):
        pyprevent.calculate_10_yr_heart_failure_risk(
            "female", 40, 129, 50, 120, True, True, 25, 70, True, True
        )


def test_invalid_hdl_cholesterol():
    with pytest.raises(ValueError):
        pyprevent.calculate_10_yr_heart_failure_risk(
            "female", 40, 200, 19, 120, True, True, 25, 70, True, True
        )


def test_invalid_systolic_bp():
    with pytest.raises(ValueError):
        pyprevent.calculate_10_yr_heart_failure_risk(
            "female", 40, 200, 50, 89, True, True, 25, 70, True, True
        )


def test_invalid_bmi():
    with pytest.raises(ValueError):
        pyprevent.calculate_10_yr_heart_failure_risk(
            "female", 40, 200, 50, 120, True, True, 18.4, 70, True, True
        )


def test_invalid_egfr():
    with pytest.raises(ValueError):
        pyprevent.calculate_10_yr_heart_failure_risk(
            "female", 40, 200, 50, 120, True, True, 25, 14.9, True, True
        )


def test_invalid_sex():
    with pytest.raises(ValueError):
        pyprevent.calculate_10_yr_heart_failure_risk(
            "unknown", 40, 200, 50, 120, True, True, 25, 70, True, True
        )


def test_calculate_30_yr_heart_failure_basic():
    # Test with some basic input values
    result = pyprevent.calculate_30_yr_heart_failure_risk(
        "female", 40, 200, 50, 120, True, True, 25, 70, True, True
    )
    # Check if the result is as expected
    assert result == pytest.approx(26.0, abs=0.1)

    result = pyprevent.calculate_30_yr_heart_failure_risk(
        "MALE", 40, 200, 50, 120, True, True, 25, 70, True, True
    )
    # Check if the result is as expected
    assert result == pytest.approx(23.5, abs=0.1)


def test_invalid_age_30():
    with pytest.raises(ValueError):
        pyprevent.calculate_30_yr_heart_failure_risk(
            "female", 29, 200, 50, 120, True, True, 25, 70, True, True
        )


def test_invalid_total_cholesterol_30():
    with pytest.raises(ValueError):
        pyprevent.calculate_30_yr_heart_failure_risk(
            "female", 40, 129, 50, 120, True, True, 25, 70, True, True
        )


def test_invalid_hdl_cholesterol_30():
    with pytest.raises(ValueError):
        pyprevent.calculate_30_yr_heart_failure_risk(
            "female", 40, 200, 19, 120, True, True, 25, 70, True, True
        )


def test_invalid_systolic_bp_30():
    with pytest.raises(ValueError):
        pyprevent.calculate_30_yr_heart_failure_risk(
            "female", 40, 200, 50, 89, True, True, 25, 70, True, True
        )


def test_invalid_bmi_30():
    with pytest.raises(ValueError):
        pyprevent.calculate_30_yr_heart_failure_risk(
            "female", 40, 200, 50, 120, True, True, 18.4, 70, True, True
        )


def test_invalid_egfr_30():
    with pytest.raises(ValueError):
        pyprevent.calculate_30_yr_heart_failure_risk(
            "female", 40, 200, 50, 120, True, True, 25, 14.9, True, True
        )


def test_invalid_sex_30():
    with pytest.raises(ValueError):
        pyprevent.calculate_30_yr_heart_failure_risk(
            "unknown", 40, 200, 50, 120, True, True, 25, 70, True, True
        )


def test_batch_calculate_10_yr_heart_failure_risk():
    test_patient = ("female", 40, 200, 50, 120, True, True, 25, 70, True, True)
    test_list = [test_patient for _ in range(10)]
    df = pd.DataFrame(
        test_list,
        columns=[
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
        ],
    )

    # Call the function
    result = pyprevent.batch_calculate_10_yr_heart_failure_risk(df)

    # Expected result
    # Regression value after the upstream d0185be coefficient correction.
    expected_result = [4.3114057073104215] * 10

    # Assertion
    np.testing.assert_allclose(result, expected_result, rtol=1e-13, atol=1e-13)


def test_batch_calculate_30_yr_heart_failure_risk():
    test_patient = ("female", 40, 200, 50, 120, True, True, 25, 70, True, True)
    test_list = [test_patient for _ in range(10)]
    df = pd.DataFrame(
        test_list,
        columns=[
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
        ],
    )

    # Call the function
    result = pyprevent.batch_calculate_30_yr_heart_failure_risk(df)

    # Expected result
    expected_result = [26.005234843826347] * 10

    # Assertion
    np.testing.assert_allclose(result, expected_result, rtol=1e-13, atol=1e-13)
