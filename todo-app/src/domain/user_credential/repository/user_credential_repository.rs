use std::fmt::Debug;

use crate::domain::{user::value_object::UserId, user_credential::entity::UserCredential};

#[async_trait::async_trait]
pub trait UserCredentialRepository: Debug + Send + Sync {
    async fn find(&self, user_id: &UserId) -> Result<Option<UserCredential>, ()>;

    async fn insert(&self, user_credential: &UserCredential) -> Result<(), ()>;

    async fn update(&self, user_credential: &UserCredential) -> Result<(), ()>;

    async fn delete(&self, user_id: &UserId) -> Result<(), ()>;
}
