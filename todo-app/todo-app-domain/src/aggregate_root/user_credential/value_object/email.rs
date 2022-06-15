use crate::error::ValidationError;

const EMAIL_MAX_LENGTH: usize = 254;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Email(String);

impl Email {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<String> for Email {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Self::Error::Required);
        }

        if value.len() > EMAIL_MAX_LENGTH {
            return Err(Self::Error::Length {
                min: None,
                max: Some(EMAIL_MAX_LENGTH),
            });
        }

        if !validator::validate_email(&value) {
            return Err(Self::Error::Email);
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

    fn email(value: &str) -> Email {
        Email::try_from(value.to_owned()).unwrap()
    }

    #[test]
    fn email_try_from_test() {
        let tests = vec![
            ("", Err(ValidationError::Required)),
            ("invalid", Err(ValidationError::Email)),
            ("ok@example.com", Ok(email("ok@example.com"))),
            (
                "valid.email.address.xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx@example.com",
                Ok(email("valid.email.address.xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx@example.com"))
            ),
            (
                "invalid.too.long.email.address.xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx@example.com",
                Err(ValidationError::Length{ min: None,  max: Some(EMAIL_MAX_LENGTH)})
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
