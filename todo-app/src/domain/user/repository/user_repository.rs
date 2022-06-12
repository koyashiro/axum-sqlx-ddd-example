use std::fmt::Debug;

use async_trait::async_trait;
use mockall::automock;

use crate::domain::user::{entity::User, value_object::UserId};

#[async_trait]
#[automock]
pub trait UserRepository: Debug + Send + Sync {
    async fn find(&mut self, user_id: &UserId) -> Result<Option<User>, ()>;

    async fn insert(&mut self, user: &User) -> Result<(), ()>;

    async fn update(&mut self, user: &User) -> Result<(), ()>;

    async fn delete(&mut self, user_id: &UserId) -> Result<(), ()>;
}
