use getset::{Getters, Setters};

use crate::aggregate_root::{
    user::value_object::UserId,
    user_credential::value_object::{Email, Password, PasswordHash},
};

#[derive(Clone, Debug, Eq, Getters, PartialEq, Setters)]
pub struct UserCredential {
    #[getset(get = "pub")]
    user_id: UserId,
    #[getset(get = "pub", set = "pub")]
    email: Email,
    #[getset(get = "pub", set = "pub")]
    password_hash: PasswordHash,
}

impl UserCredential {
    pub fn new(user_id: UserId, email: Email, password_hash: PasswordHash) -> Self {
        Self {
            user_id,
            email,
            password_hash,
        }
    }

    pub fn set_password(&mut self, password: Password) {
        self.password_hash = password.to_hash()
    }

    pub fn into_raw(self) -> (UserId, Email, PasswordHash) {
        (self.user_id, self.email, self.password_hash)
    }
}

impl From<(UserId, Email, PasswordHash)> for UserCredential {
    fn from((user_id, email, password_hash): (UserId, Email, PasswordHash)) -> Self {
        Self {
            user_id,
            email,
            password_hash,
        }
    }
}
