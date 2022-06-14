use getset::Getters;

use crate::aggregate_root::todo::value_object::{TodoId, TodoTitle};

#[derive(Clone, Debug, Eq, Getters, PartialEq)]
pub struct Todo {
    #[getset(get = "pub")]
    id: TodoId,
    #[getset(get = "pub", set = "pub")]
    title: TodoTitle,
}

impl Todo {
    pub fn new(title: TodoTitle) -> Self {
        Self {
            id: TodoId::new(),
            title,
        }
    }

    pub fn unwrap(self) -> (TodoId, TodoTitle) {
        (self.id, self.title)
    }
}

impl From<(TodoId, TodoTitle)> for Todo {
    fn from((id, title): (TodoId, TodoTitle)) -> Self {
        Self { id, title }
    }
}
