use std::fmt::Debug;

use async_trait::async_trait;
use mockall::automock;

use crate::aggregate_root::{user::value_object::UserId, user_credential::entity::UserCredential};

#[async_trait]
#[automock]
pub trait UserCredentialRepository: Debug + Send + Sync {
    async fn find(&self, user_id: &UserId) -> Result<Option<UserCredential>, anyhow::Error>;

    async fn find_by_email(&self, email: &str) -> Result<Option<UserCredential>, anyhow::Error>;

    async fn insert(&self, user_credential: &UserCredential) -> Result<(), anyhow::Error>;

    async fn update(&self, user_credential: &UserCredential) -> Result<(), anyhow::Error>;

    async fn delete(&self, user_id: &UserId) -> Result<(), anyhow::Error>;
}
