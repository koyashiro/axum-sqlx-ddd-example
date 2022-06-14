use crate::error::ValidationError;

const USER_NAME_MAX_LENGTH: usize = 30;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UserName(String);

impl UserName {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for UserName {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Self::Error::Required);
        }

        if value.len() > USER_NAME_MAX_LENGTH {
            return Err(Self::Error::Length {
                min: None,
                max: Some(USER_NAME_MAX_LENGTH),
            });
        }

        Ok(Self(value))
    }
}

impl From<UserName> for String {
    fn from(value: UserName) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use crate::error::ValidationError;

    use super::*;

    #[test]
    fn user_name_try_from() {
        let tests = vec![
            ("", Err(ValidationError::Required)),
            (
                "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
                Ok(UserName("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_owned())),
            ),
            (
                "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
                Err(ValidationError::Length {
                    min: None,
                    max: Some(USER_NAME_MAX_LENGTH),
                }),
            ),
        ];

        for (input, expected) in tests {
            assert_eq!(
                UserName::try_from(input.to_owned()),
                expected,
                "input: {input}"
            );
        }
    }
}
