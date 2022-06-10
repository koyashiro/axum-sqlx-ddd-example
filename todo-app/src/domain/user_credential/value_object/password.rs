use regex::Regex;

use crate::domain::{error::ValidationError, user_credential::value_object::PasswordHash};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Password(String);

impl Password {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn to_hash(&self) -> PasswordHash {
        PasswordHash::new(self)
    }
}

impl TryFrom<String> for Password {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Self::Error::required(value));
        }

        if !Regex::new("^[!-~]{8,128}$").unwrap().is_match(&value) {
            return Err(Self::Error::password(value));
        }

        Ok(Self(value))
    }
}

impl From<Password> for String {
    fn from(value: Password) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn password_try_from_test() {
        let tests = vec![
            ("", Err(ValidationError::required("".to_owned()))),
            (
                "passwor",
                Err(ValidationError::password("passwor".to_owned())),
            ),
            (
                "abcdefghijklmnopqrstuvwxyz",
                Ok(Password("abcdefghijklmnopqrstuvwxyz".to_owned())),
            ),
            (
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
                Ok(Password("ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_owned())),
            ),
            ("0123456789", Ok(Password("0123456789".to_owned()))),
            (
                "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~",
                Ok(Password("!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".to_owned())),
            ),
        ];

        for (input, expected) in tests {
            assert_eq!(
                Password::try_from(input.to_owned()),
                expected,
                "input: {input}"
            );
        }
    }
}
