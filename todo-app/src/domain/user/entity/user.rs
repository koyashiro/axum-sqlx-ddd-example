use nameof::name_of;
use uuid::Uuid;

use crate::domain::{
    error::ValidationErrors,
    user::value_object::{UserId, UserName},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserDto {
    pub id: Uuid,
    pub name: String,
}

impl From<User> for UserDto {
    fn from(value: User) -> Self {
        UserDto {
            id: value.id.into(),
            name: value.name.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserDtoRef<'a> {
    pub id: &'a Uuid,
    pub name: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct User {
    id: UserId,
    name: UserName,
}

impl User {
    pub fn new(id: UserId, name: UserName) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> &UserId {
        &self.id
    }

    pub fn name(&self) -> &UserName {
        &self.name
    }

    pub fn set_name(&mut self, name: UserName) {
        self.name = name
    }

    pub fn as_dto(&self) -> UserDtoRef {
        UserDtoRef {
            id: self.id.as_uuid(),
            name: self.name.as_str(),
        }
    }
}

impl TryFrom<UserDto> for User {
    type Error = ValidationErrors;

    fn try_from(value: UserDto) -> Result<Self, Self::Error> {
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

    use crate::domain::error::ValidationError;

    use super::*;

    #[test]
    fn todo_try_from_test() {
        let tests = vec![
            (
                UserDto {
                    id: "f422ff5e-f12b-43ae-80ff-a553909c8e8e".parse().unwrap(),
                    name: "".to_owned(),
                },
                Err(hashmap! {
                    "name" => ValidationError::required("".to_owned())
                }),
            ),
            (
                UserDto {
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
