use nameof::name_of;
use uuid::Uuid;

use crate::domain::{
    error::ValidationErrors,
    user::value_object::UserId,
    user_credential::value_object::{Email, Password, PasswordHash},
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserCredentialDto {
    pub user_id: Uuid,
    pub email: String,
    pub password: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserCredentialHashedDto {
    pub user_id: Uuid,
    pub email: String,
    pub password_hash: String,
}

impl From<UserCredential> for UserCredentialHashedDto {
    fn from(value: UserCredential) -> Self {
        Self {
            user_id: value.user_id.into(),
            email: value.email.into(),
            password_hash: value.password_hash.into(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserCredential {
    user_id: UserId,
    email: Email,
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

    pub fn user_id(&self) -> &UserId {
        &self.user_id
    }

    pub fn email(&self) -> &Email {
        &self.email
    }

    pub fn password_hash(&self) -> &PasswordHash {
        &self.password_hash
    }

    pub fn set_password(&mut self, password: Password) {
        self.password_hash = password.to_hash()
    }

    pub fn set_password_hash(&mut self, password_hash: PasswordHash) {
        self.password_hash = password_hash
    }
}

impl TryFrom<UserCredentialDto> for UserCredential {
    type Error = ValidationErrors;

    fn try_from(value: UserCredentialDto) -> Result<Self, Self::Error> {
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

impl TryFrom<UserCredentialHashedDto> for UserCredential {
    type Error = ValidationErrors;

    fn try_from(value: UserCredentialHashedDto) -> Result<Self, Self::Error> {
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
                UserCredentialHashedDto {
                    user_id: "4742cac6-5c3b-451a-b902-e2ae80e9183f".parse().unwrap(),
                    email: "".to_owned(),
                    password_hash: "$argon2id$v=19$m=4096,t=3,p=1$UENQVUg4dXU0ZXN1bmFUNg$4vHNzxNMkhxwn0XQvG7a7w".to_owned(),
                },
                Err(hashmap! {
                    "email" => ValidationError::required("".to_owned())
                }),
            ),
            (
                UserCredentialHashedDto {
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
