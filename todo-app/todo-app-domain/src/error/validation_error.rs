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

pub type ValidationErrors = HashMap<&'static str, ValidationError>;
