use uuid::Uuid;

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
