use std::sync::Arc;

use crate::domain::todo::{
    entity::Todo,
    repository::TodoRepository,
    value_object::{TodoId, TodoTitle},
};

#[derive(Debug)]
pub struct AddTodoUsecaseArgs {
    pub title: TodoTitle,
}

#[derive(Debug)]
pub struct AddTodoUsecase {
    todo_repository: Arc<dyn TodoRepository>,
}

impl AddTodoUsecase {
    pub async fn execute(&self, args: AddTodoUsecaseArgs) -> Result<Todo, ()> {
        let todo = Todo::new(TodoId::new(), args.title);
        self.todo_repository.insert(&todo).await?;

        Ok(todo)
    }
}
