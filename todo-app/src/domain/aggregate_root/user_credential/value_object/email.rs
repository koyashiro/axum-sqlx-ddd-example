use std::str::FromStr;

use validator::validate_email;

use crate::domain::error::ValidationError;

const EMAIL_MAX_LENGTH: usize = 254;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl FromStr for Email {
    type Err = ValidationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        TryFrom::try_from(s.to_owned())
    }
}

impl TryFrom<String> for Email {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Self::Error::required(value));
        }

        if value.len() > EMAIL_MAX_LENGTH {
            return Err(Self::Error::length(None, Some(EMAIL_MAX_LENGTH), value));
        }

        if !validate_email(&value) {
            return Err(Self::Error::email(value));
        }

        Ok(Self(value))
    }
}

impl From<Email> for String {
    fn from(value: Email) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn email_try_from_test() {
        let tests = vec![
            ("", Err(ValidationError::required("".to_owned()))),
            ("invalid", Err(ValidationError::email("invalid".to_owned()))),
            ("ok@example.com", Ok(Email("ok@example.com".to_owned()))),
            (
                "valid.email.address.xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx@example.com",
                Ok(Email("valid.email.address.xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx@example.com".to_owned()))
            ),
            (
                "invalid.too.long.email.address.xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx@example.com",
                Err(ValidationError::length( None,  Some(EMAIL_MAX_LENGTH), "invalid.too.long.email.address.xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx@example.com".to_owned()))
            ),
        ];

        for (input, expected) in tests {
            assert_eq!(
                Email::try_from(input.to_owned()),
                expected,
                "input: `{input}`"
            )
        }
    }
}
