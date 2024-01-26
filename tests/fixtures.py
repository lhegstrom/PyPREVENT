from hypothesis import strategies as st

# The below patient was entered into the PREVENT web app, and had these values returned
TEST_PATIENT: dict = {
    "sex": "female",
    "age": 33.94837059452007,
    "total_cholesterol": 130.0,
    "hdl_cholesterol": 20.0,
    "systolic_bp": 199.99999999999997,
    "has_diabetes": False,
    "current_smoker": True,
    "bmi": 18.500000000000004,
    "egfr": 139.0,
    "on_htn_meds": False,
    "on_cholesterol_meds": True,
    "10_yr_cvd_expected": 9.7,
    "30_yr_cvd_expected": 45.1,
    "10_yr_ascvd_expected": 7.6,
    "30_yr_ascvd_expected": 34.6,
    "10_yr_hf_expected": 2.7,
    "30_yr_hf_expected": 15.8,
}

sex_strategy = st.sampled_from(["male", "female"])
age_strategy_10_yr = st.floats(min_value=30.0, max_value=79.0)
age_strategy_30_yr = st.floats(min_value=30.0, max_value=59.0)
total_cholesterol_strategy = st.floats(min_value=130.0, max_value=320.0)
hdl_cholesterol_strategy = st.floats(min_value=20.0, max_value=100.0)
systolic_bp_strategy = st.floats(min_value=90.0, max_value=200.0)
boolean_strategy = st.booleans()
bmi_strategy = st.floats(min_value=18.5, max_value=39.9)
egfr_strategy = st.floats(min_value=15.0, max_value=140.0)


@st.composite
def generate_10_yr_test_case(draw):
    return {
        "sex": draw(sex_strategy),
        "age": draw(age_strategy_10_yr),
        "total_cholesterol": draw(total_cholesterol_strategy),
        "hdl_cholesterol": draw(hdl_cholesterol_strategy),
        "systolic_bp": draw(systolic_bp_strategy),
        "has_diabetes": draw(boolean_strategy),
        "current_smoker": draw(boolean_strategy),
        "bmi": draw(bmi_strategy),
        "egfr": draw(egfr_strategy),
        "on_htn_meds": draw(boolean_strategy),
        "on_cholesterol_meds": draw(boolean_strategy),
    }


@st.composite
def generate_30_yr_test_case(draw):
    return {
        "sex": draw(sex_strategy),
        "age": draw(age_strategy_30_yr),
        "total_cholesterol": draw(total_cholesterol_strategy),
        "hdl_cholesterol": draw(hdl_cholesterol_strategy),
        "systolic_bp": draw(systolic_bp_strategy),
        "has_diabetes": draw(boolean_strategy),
        "current_smoker": draw(boolean_strategy),
        "bmi": draw(bmi_strategy),
        "egfr": draw(egfr_strategy),
        "on_htn_meds": draw(boolean_strategy),
        "on_cholesterol_meds": draw(boolean_strategy),
    }
