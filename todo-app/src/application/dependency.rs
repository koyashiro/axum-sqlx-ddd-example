use std::{
    fmt::Debug,
    sync::{Arc, Mutex},
};

use async_trait::async_trait;

use crate::domain::{
    user::repository::UserRepository, user_credential::repository::UserCredentialRepository,
};

#[async_trait]
pub trait Repositories: Debug + Send + Sync {
    fn user_repository(&mut self) -> Arc<Mutex<dyn UserRepository>>;
    fn user_credential_repository(&mut self) -> Arc<Mutex<dyn UserCredentialRepository>>;
}
