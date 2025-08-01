use chrono::prelude::*;

// this will be the structure that wil handle the errors
#[derive(Debug, Eq, PartialEq)]
pub struct FormError {
    // expected public fields
    form_values: (String, String),
    date: String,
    err: String,
}

impl FormError {
    pub fn new(field_name: &'static str, field_value: String, err: &'static str) -> Self {
        return FormError{
            form_values: (field_name.to_string(), field_value.to_string()),
            date: Local::now().format("%Y-%m-%d %H:%M:%S").to_string(), 
            err : err.to_string(),
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct Form {
    // expected public fields
    pub name : String,
    pub password: String,
}

impl Form {
    pub fn new(name: &str, password: &str) -> Self{
        return Form{name: name.to_string(), password: password.to_string()};
    }
    pub fn validate(&self) -> Result<(), FormError> {
        if !&self.name.len() == 0 {
            return Err(FormError::new(
                "name",
                self.name.clone(),
                "Username is empty"
            ));
        }
        if !&self.password.len() < 8 {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be at least 8 characters long"
            ));
        }
        if self.password.chars().any(|c| !c.is_alphanumeric()) {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols"
            ));
        }
        if !&self.password.chars().any(|c| c.is_numeric()) {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols"
            ));
        }
        if !&self.password.chars().any(|c| c.is_alphanumeric()) {
            return Err(FormError::new(
                "password",
                self.password.clone(),
                "Password should be a combination of ASCII numbers, letters and symbols"
            ));
        }
        Ok(())
    }
}
