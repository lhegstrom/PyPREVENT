# PREVENT Equations

###### 2024 updates from AHA on ASCVD (Atherosclerotic and Cardiovascular Disease), CVD (cardiovascular disease) and Heart Failure (HF)


## TL;DR



## Program Structure

This is a mixed [Rust](https://www.rust-lang.org/) and Python module.

The rust source code is used to implement the equations. This is a lower level language that requires compilation prior to being run -- and thus is many times faster than pure python.

The rust source code is located in the /src directory.
The individual functions are written in their respective files, and registered to the rust_aha_formulas python module within the lib.rs file.

The python source is located in the /aha_formulas directory.

Unit tests are implemented in the /tests directory.