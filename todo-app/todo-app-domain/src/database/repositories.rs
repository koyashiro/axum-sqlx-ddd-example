use std::{fmt::Debug, sync::Arc};

use crate::aggregate_root::{
    todo::repository::TodoRepository, user::repository::UserRepository,
    user_credential::repository::UserCredentialRepository,
};

pub trait Repositories: Debug + Send + Sync {
    fn user_repository(&self) -> Arc<dyn UserRepository>;
    fn user_credential_repository(&self) -> Arc<dyn UserCredentialRepository>;
    fn todo_repository(&self) -> Arc<dyn TodoRepository>;
}
