use std::sync::Arc;

use crate::domain::todo::{repository::TodoRepository, value_object::TodoId};

#[derive(Debug)]
pub struct RemoveTodoUsecaseArgs {
    pub id: TodoId,
}

#[derive(Debug)]
pub struct RemoveTodoUsecase {
    todo_repository: Arc<dyn TodoRepository>,
}

impl RemoveTodoUsecase {
    pub async fn execute(&self, args: RemoveTodoUsecaseArgs) -> Result<(), ()> {
        self.todo_repository.delete(&args.id).await?;

        Ok(())
    }
}
