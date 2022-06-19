use std::str::FromStr;

use uuid::Uuid;

use crate::error::ValidationError;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct UserId(Uuid);

impl UserId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> &Uuid {
        AsRef::as_ref(self)
    }

    pub fn into_uuid(self) -> Uuid {
        Into::into(self)
    }

    pub fn parse_str(s: &str) -> Result<UserId, ValidationError> {
        FromStr::from_str(s)
    }
}

impl AsRef<Uuid> for UserId {
    fn as_ref(&self) -> &Uuid {
        &self.0
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
