use getset::{Getters, Setters};
use nameof::name_of;
use uuid::Uuid;

use crate::domain::{
    aggregate_root::{
        user::value_object::UserId,
        user_credential::value_object::{Email, Password, PasswordHash},
    },
    error::ValidationErrors,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserCredentialRaw {
    pub user_id: Uuid,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserCredentialHashedRaw {
    pub user_id: Uuid,
    pub email: String,
    pub password_hash: String,
}

impl From<UserCredential> for UserCredentialHashedRaw {
    fn from(value: UserCredential) -> Self {
        Self {
            user_id: value.user_id.into(),
            email: value.email.into(),
            password_hash: value.password_hash.into(),
        }
    }
}

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
}

impl TryFrom<UserCredentialRaw> for UserCredential {
    type Error = ValidationErrors;

    fn try_from(value: UserCredentialRaw) -> Result<Self, Self::Error> {
        let user_id = UserId::from(value.user_id);
        let email = Email::try_from(value.email);
        let password = Password::try_from(value.password);
        match (email, password) {
            (Ok(email), Ok(password)) => Ok(Self {
                user_id,
                email,
                password_hash: password.to_hash(),
            }),
            (email, password) => {
                let mut errors = Self::Error::new();
                if let Err(email) = email {
                    errors.insert(name_of!(email), email);
                }
                if let Err(password) = password {
                    errors.insert(name_of!(password), password);
                }
                Err(errors)
            }
        }
    }
}

impl TryFrom<UserCredentialHashedRaw> for UserCredential {
    type Error = ValidationErrors;

    fn try_from(value: UserCredentialHashedRaw) -> Result<Self, Self::Error> {
        let user_id = value.user_id.into();
        let email = value.email.try_into();
        let password_hash = value.password_hash.parse();
        match (email, password_hash) {
            (Ok(email), Ok(password_hash)) => Ok(Self {
                user_id,
                email,
                password_hash,
            }),
            (email, password_hash) => {
                let mut errors = Self::Error::new();
                if let Err(email) = email {
                    errors.insert(name_of!(email), email);
                }
                if let Err(password_hash) = password_hash {
                    errors.insert(name_of!(password_hash), password_hash);
                }
                Err(errors)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;

    use crate::domain::error::ValidationError;

    use super::*;

    #[test]
    fn todo_try_from_test() {
        let tests = vec![
            (
                UserCredentialHashedRaw {
                    user_id: "4742cac6-5c3b-451a-b902-e2ae80e9183f".parse().unwrap(),
                    email: "".to_owned(),
                    password_hash: "$argon2id$v=19$m=4096,t=3,p=1$UENQVUg4dXU0ZXN1bmFUNg$4vHNzxNMkhxwn0XQvG7a7w".to_owned(),
                },
                Err(hashmap! {
                    "email" => ValidationError::required("".to_owned())
                }),
            ),
            (
                UserCredentialHashedRaw {
                    user_id: "fd71d85c-c88b-4844-91b9-ddafb5bf34e4".parse().unwrap(),
                    email: "user@example.com".to_owned(),
                    password_hash: "$argon2id$v=19$m=4096,t=3,p=1$WVl6RWp6OVNxNnhNVlAydw$DUv40d2Qjq338qWivCcakw".to_owned(),
                },
                Ok(UserCredential {
                    user_id: "fd71d85c-c88b-4844-91b9-ddafb5bf34e4".parse::<Uuid>().unwrap().into(),
                    email: "user@example.com".to_owned().try_into().unwrap(),
                    password_hash: "$argon2id$v=19$m=4096,t=3,p=1$WVl6RWp6OVNxNnhNVlAydw$DUv40d2Qjq338qWivCcakw".parse().unwrap(),
                }),
            ),
        ];

        for (input, expected) in tests {
            assert_eq!(UserCredential::try_from(input), expected);
        }
    }
}
