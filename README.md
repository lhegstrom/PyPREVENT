# PyPREVENT :anatomical_heart:


###### 2024 updates from AHA on ASCVD (Atherosclerotic and Cardiovascular Disease), CVD (cardiovascular disease) and Heart Failure (HF)

![Python- 3.7 --> 3.12](https://img.shields.io/badge/python-3670A0?style=for-the-badge&logo=python&logoColor=ffdd54)
![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)

## Introduction
The PyPREVENT Equations package offers 2024 updates from the [American Heart Association (AHA) on Atherosclerotic and
Cardiovascular Disease (ASCVD), Cardiovascular Disease (CVD), and Heart Failure
(HF)](https://professional.heart.org/en/guidelines-and-statements/prevent-calculator)[^1][^2]. It's a mixed Rust and Python
module, leveraging the speed of Rust for equation implementation and the flexibility of Python for ease of use.

## Installation
**Requirements:**
- Python 3.7 to 3.12 on a Silicon Mac / Linux system (more compatibility coming soon)


To install the package, pip install using:
```bash
pip install pyprevent
```

## TL;DR

```python
import pyprevent

pyprevent.calculate_30_yr_ascvd_risk(
    sex="MALE",
    age=40,
    total_cholesterol=200,
    hdl_cholesterol=50,
    systolic_bp=120,
    has_diabetes=False,
    current_smoker=False,
    bmi=25,
    egfr=70,
    on_htn_meds=False,
    on_cholesterol_meds=False,
)
```

[A longer, and more thorough example is located here.](example_notebooks/Example%20Notebook.ipynb)

## Implementation Status

| Formula                          | Status             |
|----------------------------------|--------------------|
| 10 yr ASCVD (individual)         | :white_check_mark: |
| 10 yr ASCVD (batch)              | :white_check_mark: |
| 30 yr ASCVD (individual)         | :white_check_mark: |
| 30 yr ASCVD (batch)              | :white_check_mark: |
| 10 yr Heart Failure (individual) | :white_check_mark: |
| 10 yr Heart Failure (batch)      | :white_check_mark: |
| 30 yr Heart Failure (individual) | :white_check_mark: |
| 30 yr Heart Failure (batch)      | :white_check_mark: |
| 10 yr CVD (individual)           | :white_check_mark: |
| 10 yr CVD (batch)                | :white_check_mark: |
| 30 yr CVD (individual)           | :white_check_mark: |
| 30 yr CVD (batch)                | :white_check_mark: |


## Program Structure

This is a mixed [Rust](https://www.rust-lang.org/) and Python module.

The rust source code is used to implement the equations. This is a lower level language that requires compilation prior to being run -- and thus is many times faster than pure python.

The rust source code is located in the /src directory.
The individual functions are written in their respective files, and registered to the rust_aha_formulas python module within the lib.rs file.

The python source is located in the /pyprevent directory.

Unit tests are implemented in the /tests directory using [slash](https://getslash.github.io/slash/).


[^1]: Khan SS, Matsushita K, Sang Y, et al. Development and Validation of the American Heart Association Predicting Risk of Cardiovascular Disease EVENTs (PREVENTTM) Equations. Circulation 2023. DOI: [10.1161/CIRCULATIONAHA.123.067626](https://www.ahajournals.org/doi/10.1161/CIRCULATIONAHA.123.067626)
[^2]: Khan SS, Coresh J, Pencina MJ, et al. Novel Prediction Equations for Absolute Risk Assessment of Total Cardiovascular Disease Incorporating Cardiovascular-Kidney-Metabolic Health: A Scientific Statement From the American Heart Association. Circulation 2023;148(24):1982-2004. DOI: [10.1161/CIR.0000000000001191](https://www.ahajournals.org/doi/10.1161/CIR.0000000000001191)

