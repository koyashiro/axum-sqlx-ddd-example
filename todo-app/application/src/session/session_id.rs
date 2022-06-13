use uuid::Uuid;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SessionId(Uuid);

impl SessionId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn as_uuid(&self) -> &Uuid {
        &self.0
    }

    pub fn to_simple_string(&self) -> String {
        self.0.to_simple_ref().to_string()
    }
}

impl From<Uuid> for SessionId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}
