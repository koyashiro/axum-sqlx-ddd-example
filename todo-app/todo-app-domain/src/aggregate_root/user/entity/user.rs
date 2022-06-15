use getset::{Getters, Setters};

use crate::aggregate_root::user::value_object::{UserId, UserName};

#[derive(Clone, Debug, Eq, Getters, PartialEq, Setters)]
pub struct User {
    #[getset(get = "pub")]
    id: UserId,
    #[getset(get = "pub", set = "pub")]
    name: UserName,
}

impl User {
    pub fn new(name: UserName) -> Self {
        Self {
            id: UserId::new(),
            name,
        }
    }

    pub fn into_inner(self) -> (UserId, UserName) {
        (self.id, self.name)
    }
}

impl From<(UserId, UserName)> for User {
    fn from((id, name): (UserId, UserName)) -> Self {
        Self { id, name }
    }
}
