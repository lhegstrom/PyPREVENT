import slash
import pyprevent
import pandas as pd


def test_calculate_10_yr_ascvd_risk_basic():
    result = pyprevent.calculate_10_yr_ascvd_risk(
        "female", 40, 200, 50, 120, True, True, 25, 70, True, True
    )
    slash.assert_almost_equal(result, 4.7, delta=0.1)

    result = pyprevent.calculate_10_yr_ascvd_risk(
        "MALE", 68, 300, 85, 150, False, True, 35, 65, False, True
    )
    slash.assert_almost_equal(result, 12.9, delta=0.1)


def test_invalid_age():
    with slash.assert_raises(ValueError):
        pyprevent.calculate_10_yr_ascvd_risk(
            "female", 29, 200, 50, 120, True, True, 25, 70, True, True
        )


def test_invalid_total_cholesterol():
    with slash.assert_raises(ValueError):
        pyprevent.calculate_10_yr_ascvd_risk(
            "female", 40, 129, 50, 120, True, True, 25, 70, True, True
        )


def test_invalid_hdl_cholesterol():
    with slash.assert_raises(ValueError):
        pyprevent.calculate_10_yr_ascvd_risk(
            "female", 40, 200, 19, 120, True, True, 25, 70, True, True
        )


def test_invalid_systolic_bp():
    with slash.assert_raises(ValueError):
        pyprevent.calculate_10_yr_ascvd_risk(
            "female", 40, 200, 50, 89, True, True, 25, 70, True, True
        )


def test_invalid_bmi():
    with slash.assert_raises(ValueError):
        pyprevent.calculate_10_yr_ascvd_risk(
            "female", 40, 200, 50, 120, True, True, 18.4, 70, True, True
        )


def test_invalid_egfr():
    with slash.assert_raises(ValueError):
        pyprevent.calculate_10_yr_ascvd_risk(
            "female", 40, 200, 50, 120, True, True, 25, 14.9, True, True
        )


def test_invalid_sex():
    with slash.assert_raises(ValueError):
        pyprevent.calculate_10_yr_ascvd_risk(
            "unknown", 40, 200, 50, 120, True, True, 25, 70, True, True
        )


def test_calculate_30_yr_heart_failure_basic():
    # Test with some basic input values
    result = pyprevent.calculate_30_yr_ascvd_risk(
        "female", 40, 200, 50, 120, True, True, 25, 70, True, True
    )
    # Check if the result is as expected
    slash.assert_almost_equal(result, 23.4, delta=0.1)

    result = pyprevent.calculate_30_yr_ascvd_risk(
        "MALE", 40, 200, 50, 120, True, True, 25, 70, True, True
    )
    # Check if the result is as expected
    slash.assert_almost_equal(result, 23.7, delta=0.1)


def test_invalid_age_30():
    with slash.assert_raises(ValueError):
        pyprevent.calculate_30_yr_ascvd_risk(
            "female", 29, 200, 50, 120, True, True, 25, 70, True, True
        )


def test_invalid_total_cholesterol_30():
    with slash.assert_raises(ValueError):
        pyprevent.calculate_30_yr_ascvd_risk(
            "female", 40, 129, 50, 120, True, True, 25, 70, True, True
        )


def test_invalid_hdl_cholesterol_30():
    with slash.assert_raises(ValueError):
        pyprevent.calculate_30_yr_ascvd_risk(
            "female", 40, 200, 19, 120, True, True, 25, 70, True, True
        )


def test_invalid_systolic_bp_30():
    with slash.assert_raises(ValueError):
        pyprevent.calculate_30_yr_ascvd_risk(
            "female", 40, 200, 50, 89, True, True, 25, 70, True, True
        )


def test_invalid_bmi_30():
    with slash.assert_raises(ValueError):
        pyprevent.calculate_30_yr_ascvd_risk(
            "female", 40, 200, 50, 120, True, True, 18.4, 70, True, True
        )


def test_invalid_egfr_30():
    with slash.assert_raises(ValueError):
        pyprevent.calculate_30_yr_ascvd_risk(
            "female", 40, 200, 50, 120, True, True, 25, 14.9, True, True
        )


def test_invalid_sex_30():
    with slash.assert_raises(ValueError):
        pyprevent.calculate_30_yr_ascvd_risk(
            "unknown", 40, 200, 50, 120, True, True, 25, 70, True, True
        )


def test_batch_calculate_10_yr_heart_failure_risk():
    # Test data setup
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
    result = pyprevent.batch_calculate_10_yr_ascvd_risk(df)

    # Expected result
    expected_result = [4.723678963112583] * 10

    # Assertion
    assert result == expected_result


def test_batch_calculate_30_yr_heart_failure_risk():

    # Test data setup
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
    result = pyprevent.batch_calculate_30_yr_ascvd_risk(df)

    # Expected result
    expected_result = [23.4074103963271] * 10

    # Assertion
    assert result == expected_result
