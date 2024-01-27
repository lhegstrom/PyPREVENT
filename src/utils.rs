use std::f64;

pub fn validate_input(
    age: f64,
    total_cholesterol: f64,
    hdl_cholesterol: f64,
    systolic_bp: f64,
    bmi: f64,
    egfr: f64,
    ten_year: bool,
) -> Result<(), String> {

    if ten_year { validate_10_yr_age(&age)? } else { validate_30_yr_age(&age)? }

    if !(130.0..=320.0).contains(&total_cholesterol) {
        return Err("Total cholesterol must be between 130 and 320".to_string());
    }
    if !(20.0..=100.0).contains(&hdl_cholesterol) {
        return Err("HDL cholesterol must be between 20 and 100".to_string());
    }
    if !(90.0..=200.0).contains(&systolic_bp) {
        return Err("Systolic blood pressure must be between 90 and 200".to_string());
    }
    if !(18.5..=39.9).contains(&bmi) {
        return Err("BMI must be between 18.5 and 39.9".to_string());
    }
    if !(15.0..=140.0).contains(&egfr) {
        return Err("eGFR must be between 15 and 140".to_string());
    }
    Ok(())
}

fn validate_10_yr_age(&age: &f64) -> Result<(), String> {
    if !(30.0..=79.0).contains(&age) {
        return Err("Age must be between 30 and 79".to_string());
    }
    Ok(())
}

fn validate_30_yr_age(&age: &f64) -> Result<(), String> {
    if !(30.0..=59.0).contains(&age) {
        return Err("Age must be between 30 and 59".to_string());
    }
    Ok(())
}
