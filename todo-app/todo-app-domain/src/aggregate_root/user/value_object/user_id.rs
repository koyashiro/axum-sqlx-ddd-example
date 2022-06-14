use std::str::FromStr;

use ulid::Ulid;
use uuid::Uuid;

use crate::error::ValidationError;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Ulid::new().into())
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    pub fn into_uuid(self) -> Uuid {
        self.0
    }

    pub fn parse_str(s: &str) -> Result<UserId, ValidationError> {
        FromStr::from_str(s)
    }
}

impl From<Uuid> for UserId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<UserId> for Uuid {
    fn from(value: UserId) -> Self {
        value.0
    }
}

impl FromStr for UserId {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::parse_str(s).map_err(|_| ValidationError::Invalid)?;
        let user_id = UserId::from(uuid);
        Ok(user_id)
    }
}
