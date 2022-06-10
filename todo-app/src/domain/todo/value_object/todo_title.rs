use crate::domain::error::ValidationError;

const TODO_TITLE_MAX_LENGTH: usize = 100;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TodoTitle(String);

impl TodoTitle {
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for TodoTitle {
    type Error = ValidationError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(Self::Error::required(value));
        }

        if value.len() > TODO_TITLE_MAX_LENGTH {
            return Err(Self::Error::length(
                None,
                Some(TODO_TITLE_MAX_LENGTH),
                value,
            ));
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
    use crate::domain::error::ValidationError;

    use super::*;

    #[test]
    fn todo_title_try_from() {
        let tests = vec![
            ("", Err(ValidationError::required("".to_owned()))),
            (
                "new todo title",
                Ok(TodoTitle("new todo title".to_owned())),
            ),
            (
                "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
                Ok(TodoTitle("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_owned())),
            ),
            (
                "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
                Err(ValidationError::length(
                    None,
                    Some(TODO_TITLE_MAX_LENGTH),
                    "xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx".to_owned(),
                )),
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
