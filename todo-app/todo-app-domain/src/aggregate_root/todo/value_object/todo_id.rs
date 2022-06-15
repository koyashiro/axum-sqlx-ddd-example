use ulid::Ulid;
use uuid::Uuid;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct TodoId(Uuid);

impl TodoId {
    pub fn new() -> Self {
        Self(Ulid::new().into())
    }

    pub fn as_uuid(&self) -> &Uuid {
        AsRef::as_ref(self)
    }

    pub fn into_uuid(self) -> Uuid {
        Into::into(self)
    }
}

impl AsRef<Uuid> for TodoId {
    fn as_ref(&self) -> &Uuid {
        &self.0
    }
}

impl From<Uuid> for TodoId {
    fn from(value: Uuid) -> Self {
        Self(value)
    }
}

impl From<TodoId> for Uuid {
    fn from(value: TodoId) -> Self {
        value.0
    }
}
