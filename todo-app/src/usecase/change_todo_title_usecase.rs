use std::sync::Arc;

use crate::domain::todo::{
    entity::Todo,
    repository::TodoRepository,
    value_object::{TodoId, TodoTitle},
};

#[derive(Debug)]
pub struct ChangeTodoTitleUsecaseArgs {
    pub id: TodoId,
    pub title: TodoTitle,
}

#[derive(Debug)]
pub struct ChangeTodoTitleUsecase {
    todo_repository: Arc<dyn TodoRepository>,
}

impl ChangeTodoTitleUsecase {
    pub async fn execute(&self, args: ChangeTodoTitleUsecaseArgs) -> Result<Todo, ()> {
        let mut todo = self.todo_repository.find(&args.id).await?.ok_or(())?;
        todo.set_title(args.title);
        self.todo_repository.update(&todo).await?;

        Ok(todo)
    }
}
