use std::fmt::Debug;

use async_trait::async_trait;
use mockall::automock;

use crate::aggregate_root::user::{entity::User, value_object::UserId};

#[async_trait]
#[automock]
pub trait UserRepository: Debug + Send + Sync {
    async fn find(&self, user_id: &UserId) -> Result<Option<User>, anyhow::Error>;

    async fn insert(&self, user: &User) -> Result<(), anyhow::Error>;

    async fn update(&self, user: &User) -> Result<(), anyhow::Error>;

    async fn delete(&self, user_id: &UserId) -> Result<(), anyhow::Error>;
}
