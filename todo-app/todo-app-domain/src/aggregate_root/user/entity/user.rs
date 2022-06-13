use getset::{Getters, Setters};
use nameof::name_of;
use uuid::Uuid;

use crate::{
    aggregate_root::user::value_object::{UserId, UserName},
    error::ValidationErrors,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserRaw {
    pub id: Uuid,
    pub name: String,
}

impl From<User> for UserRaw {
    fn from(value: User) -> Self {
        UserRaw {
            id: value.id.into(),
            name: value.name.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserRawRef<'a> {
    pub id: &'a Uuid,
    pub name: &'a str,
}

#[derive(Clone, Debug, Eq, Getters, PartialEq, Setters)]
pub struct User {
    #[getset(get = "pub")]
    id: UserId,
    #[getset(get = "pub", set = "pub")]
    name: UserName,
}

impl User {
    pub fn new(id: UserId, name: UserName) -> Self {
        Self { id, name }
    }

    pub fn as_raw(&self) -> UserRawRef {
        UserRawRef {
            id: self.id.as_uuid(),
            name: self.name.as_str(),
        }
    }
}

impl TryFrom<UserRaw> for User {
    type Error = ValidationErrors;

    fn try_from(value: UserRaw) -> Result<Self, Self::Error> {
        let id = value.id.into();
        let name = value.name.try_into();
        match name {
            Ok(name) => Ok(Self { id, name }),
            Err(name) => {
                let mut errors = Self::Error::new();
                errors.insert(name_of!(name), name);
                Err(errors)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use maplit::hashmap;
    use uuid::Uuid;

    use crate::error::ValidationError;

    use super::*;

    #[test]
    fn todo_try_from_test() {
        let tests = vec![
            (
                UserRaw {
                    id: "f422ff5e-f12b-43ae-80ff-a553909c8e8e".parse().unwrap(),
                    name: "".to_owned(),
                },
                Err(hashmap! {
                    "name" => ValidationError::required("".to_owned())
                }),
            ),
            (
                UserRaw {
                    id: "d91494c1-87ce-46bf-a816-abf2f5c09bc0".parse().unwrap(),
                    name: "user name".to_owned(),
                },
                Ok(User {
                    id: "d91494c1-87ce-46bf-a816-abf2f5c09bc0"
                        .parse::<Uuid>()
                        .unwrap()
                        .into(),
                    name: "user name".to_owned().try_into().unwrap(),
                }),
            ),
        ];

        for (input, expected) in tests {
            assert_eq!(User::try_from(input), expected);
        }
    }
}
