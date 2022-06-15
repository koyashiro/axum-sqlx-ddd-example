use regex::Regex;

use crate::{aggregate_root::user_credential::value_object::PasswordHash, error::ValidationError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Password(String);

impl Password {
    pub fn as_str(&self) -> &str {
        AsRef::as_ref(self)
    }

    pub fn into_string(self) -> String {
        Into::into(self)
    }

    pub fn to_hash(&self) -> PasswordHash {
        PasswordHash::new(self)
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for Password {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Self::Error::Required);
        }

        if !Regex::new("^[!-~]{8,128}$").unwrap().is_match(&value) {
            return Err(Self::Error::Password);
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

    fn password(value: &str) -> Password {
        Password::try_from(value.to_owned()).unwrap()
    }

    #[test]
    fn password_try_from_test() {
        let tests = vec![
            ("", Err(ValidationError::Required)),
            ("passwor", Err(ValidationError::Required)),
            (
                "abcdefghijklmnopqrstuvwxyz",
                Ok(password("abcdefghijklmnopqrstuvwxyz")),
            ),
            (
                "ABCDEFGHIJKLMNOPQRSTUVWXYZ",
                Ok(password("ABCDEFGHIJKLMNOPQRSTUVWXYZ")),
            ),
            ("0123456789", Ok(password("0123456789"))),
            (
                "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~",
                Ok(password("!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~")),
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
