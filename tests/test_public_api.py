import pytest

import pyprevent

from .fixtures import BASELINE_PATIENT, build_patient

SCALAR_FUNCTION_NAMES = [
    f"calculate_{years}_yr_{model}_risk"
    for years in (10, 30)
    for model in ("ascvd", "cvd", "heart_failure")
]


@pytest.mark.parametrize("function_name", SCALAR_FUNCTION_NAMES)
def test_scalar_functions_accept_positional_arguments(function_name: str) -> None:
    scalar_function = getattr(pyprevent, function_name)
    positional_risk = scalar_function(*BASELINE_PATIENT.values())
    assert positional_risk == scalar_function(**BASELINE_PATIENT)


@pytest.mark.parametrize("function_name", SCALAR_FUNCTION_NAMES)
def test_scalar_functions_reject_wrong_input_types(function_name: str) -> None:
    scalar_function = getattr(pyprevent, function_name)
    with pytest.raises(TypeError):
        scalar_function(**build_patient(has_diabetes=1))
    with pytest.raises(TypeError):
        scalar_function(**build_patient(age="40"))


def test_package_exports_only_public_risk_functions() -> None:
    assert sorted(pyprevent.__all__) == sorted(
        [f"batch_{name}" for name in SCALAR_FUNCTION_NAMES] + SCALAR_FUNCTION_NAMES
    )
    for exported_name in pyprevent.__all__:
        assert callable(getattr(pyprevent, exported_name))
