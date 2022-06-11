use std::collections::HashMap;

use thiserror::Error;

#[derive(Clone, Debug, Error, PartialEq)]
#[error("validation error")]
pub struct ValidationError {
    kind: ValidationErrorKind,
    value: Value,
}

impl ValidationError {
    pub fn new(kind: ValidationErrorKind, value: Value) -> Self {
        Self { kind, value }
    }

    pub fn required(value: String) -> Self {
        Self {
            kind: ValidationErrorKind::Required,
            value: Value::String(value),
        }
    }

    pub fn length(min: Option<usize>, max: Option<usize>, value: String) -> Self {
        Self {
            kind: ValidationErrorKind::Length { min, max },
            value: Value::String(value),
        }
    }

    pub fn email(value: String) -> Self {
        Self {
            kind: ValidationErrorKind::Email,
            value: Value::String(value),
        }
    }

    pub fn password(value: String) -> Self {
        Self {
            kind: ValidationErrorKind::Password,
            value: Value::String(value),
        }
    }

    pub fn password_hash(source: argon2::password_hash::Error, value: String) -> Self {
        Self {
            kind: ValidationErrorKind::PasswordHash(source),
            value: Value::String(value),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ValidationErrorKind {
    Required,
    Length {
        min: Option<usize>,
        max: Option<usize>,
    },
    Email,
    Password,
    PasswordHash(argon2::password_hash::Error),
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Bool(bool),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    String(String),
    Vec(Vec<Value>),
}

pub type ValidationErrors = HashMap<&'static str, ValidationError>;
