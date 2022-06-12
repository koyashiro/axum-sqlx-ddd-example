use std::fmt::Debug;

use async_trait::async_trait;
use mockall::automock;

use crate::domain::{
    todo::{entity::Todo, value_object::TodoId},
    user::value_object::UserId,
};

#[async_trait]
#[automock]
pub trait TodoRepository: Debug + Send + Sync {
    async fn find(&self, todo_id: &TodoId) -> Result<Option<Todo>, ()>;

    async fn find_by_user_id(&self, user_id: &UserId) -> Result<Vec<Todo>, ()>;

    async fn insert(&self, todo: &Todo) -> Result<(), ()>;

    async fn update(&self, todo: &Todo) -> Result<(), ()>;

    async fn delete(&self, todo_id: &TodoId) -> Result<(), ()>;
}
