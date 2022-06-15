use std::collections::HashMap;

use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq)]
#[error("validation error")]
pub enum ValidationError {
    Required,
    Length {
        min: Option<usize>,
        max: Option<usize>,
    },
    Email,
    Password,
    PasswordHash(argon2::password_hash::Error),
    Invalid,
}

#[derive(Debug, Default, Error)]
#[error("validation error")]
pub struct ValidationErrors(HashMap<&'static str, ValidationError>);

impl ValidationErrors {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn builder() -> ValidationErrorsBuilder {
        Default::default()
    }

    pub fn add(&mut self, field: &'static str, error: ValidationError) {
        self.0.insert(field, error);
    }

    pub fn into_hash_map(self) -> HashMap<&'static str, ValidationError> {
        self.0
    }
}

impl From<HashMap<&'static str, ValidationError>> for ValidationErrors {
    fn from(map: HashMap<&'static str, ValidationError>) -> Self {
        Self(map)
    }
}

impl From<ValidationErrors> for HashMap<&'static str, ValidationError> {
    fn from(errors: ValidationErrors) -> Self {
        errors.0
    }
}

#[derive(Debug, Default)]
pub struct ValidationErrorsBuilder(HashMap<&'static str, ValidationError>);

impl ValidationErrorsBuilder {
    pub fn add(mut self, field: &'static str, error: ValidationError) -> Self {
        self.0.insert(field, error);
        self
    }

    pub fn build(self) -> ValidationErrors {
        Into::into(self.0)
    }
}
