use nameof::name_of;
use uuid::Uuid;

use crate::domain::{
    error::ValidationErrors,
    todo::value_object::{TodoId, TodoTitle},
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TodoDto {
    pub id: Uuid,
    pub title: String,
}

impl From<Todo> for TodoDto {
    fn from(value: Todo) -> Self {
        TodoDto {
            id: value.id.into(),
            title: value.title.into(),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TodoDtoRef<'a> {
    pub id: &'a Uuid,
    pub title: &'a str,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Todo {
    id: TodoId,
    title: TodoTitle,
}

impl Todo {
    pub fn new(id: TodoId, title: TodoTitle) -> Self {
        Self { id, title }
    }

    pub fn id(&self) -> &TodoId {
        &self.id
    }

    pub fn title(&self) -> &TodoTitle {
        &self.title
    }

    pub fn set_title(&mut self, title: TodoTitle) {
        self.title = title
    }

    pub fn as_dto(&self) -> TodoDtoRef {
        TodoDtoRef {
            id: self.id.as_uuid(),
            title: self.title.as_str(),
        }
    }
}

impl TryFrom<TodoDto> for Todo {
    type Error = ValidationErrors;

    fn try_from(value: TodoDto) -> Result<Self, Self::Error> {
        let id = value.id.into();
        let title = value.title.try_into();
        match title {
            Ok(title) => Ok(Self { id, title }),
            Err(title) => {
                let mut errors = Self::Error::new();
                errors.insert(name_of!(title), title);
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
                TodoDto {
                    id: "e71f460e-abb6-46a5-a352-0fc24aa63143".parse().unwrap(),
                    title: "".to_owned(),
                },
                Err(hashmap! {
                    "title" => ValidationError::required("".to_owned())
                }),
            ),
            (
                TodoDto {
                    id: "8629d01e-bc1c-4560-ae8d-b5f6c2a1bce8".parse().unwrap(),
                    title: "todo title".to_owned(),
                },
                Ok(Todo {
                    id: "8629d01e-bc1c-4560-ae8d-b5f6c2a1bce8"
                        .parse::<Uuid>()
                        .unwrap()
                        .into(),
                    title: "todo title".to_owned().try_into().unwrap(),
                }),
            ),
        ];

        for (input, expected) in tests {
            assert_eq!(Todo::try_from(input), expected);
        }
    }
}
