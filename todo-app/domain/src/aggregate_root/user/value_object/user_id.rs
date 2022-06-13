use std::{fmt::Display, str::FromStr};

use uuid::Uuid;

use crate::error::ValidationError;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }
}

impl Display for UserId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<Uuid> for UserId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl FromStr for UserId {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse().map_err(|_| Self::Err::invalid(s.to_owned()))
    }
}

impl From<UserId> for Uuid {
    fn from(value: UserId) -> Self {
        value.0
    }
}
