use crate::error::ValidationError;

const TODO_TITLE_MAX_LENGTH: usize = 100;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TodoTitle(String);

impl TodoTitle {
    pub fn as_str(&self) -> &str {
        &self.0
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl TryFrom<String> for TodoTitle {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Self::Error::Required);
        }

        if value.len() > TODO_TITLE_MAX_LENGTH {
            return Err(Self::Error::Length {
                min: None,
                max: Some(TODO_TITLE_MAX_LENGTH),
            });
        }

        Ok(Self(value))
    }
}

impl From<TodoTitle> for String {
    fn from(value: TodoTitle) -> Self {
        value.0
    }
}

#[cfg(test)]
mod tests {
    use crate::{error::ValidationError, macros::*};

    use super::*;

    #[test]
    fn todo_title_try_from() {
        let tests = vec![
            ("", Err(ValidationError::Required)),
            (
                "new todo title",
                Ok(todo_title!("new todo title")),
            ),
            (
                "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
                Ok(todo_title!("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx")),
            ),
            (
                "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
                Err(ValidationError::Length{
                    min: None,
                    max: Some(TODO_TITLE_MAX_LENGTH),
                }),
            ),
        ];

        for (input, expected) in tests {
            assert_eq!(
                TodoTitle::try_from(input.to_owned()),
                expected,
                "input: {input}"
            );
        }
    }
}
