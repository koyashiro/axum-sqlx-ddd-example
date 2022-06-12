use std::fmt::Debug;

use async_trait::async_trait;
use mockall::automock;

use crate::domain::{
    user::value_object::UserId,
    user_credential::{entity::UserCredential, value_object::Email},
};

#[async_trait]
#[automock]
pub trait UserCredentialRepository: Debug + Send + Sync {
    async fn find(&mut self, user_id: &UserId) -> Result<Option<UserCredential>, ()>;

    async fn find_by_email(&mut self, email: &Email) -> Result<Option<UserCredential>, ()>;

    async fn insert(&mut self, user_credential: &UserCredential) -> Result<(), ()>;

    async fn update(&mut self, user_credential: &UserCredential) -> Result<(), ()>;

    async fn delete(&mut self, user_id: &UserId) -> Result<(), ()>;
}
