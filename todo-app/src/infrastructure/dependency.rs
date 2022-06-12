use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

use crate::{
    application::dependency::Repositories,
    domain::{
        user::repository::UserRepository, user_credential::repository::UserCredentialRepository,
    },
};

use super::database::repository::{PgUserCredentialRepository, PgUserRepository};

#[derive(Debug)]
pub struct PgRepositories {
    tx: Arc<Mutex<Transaction<'static, Postgres>>>,
    user_repository: Arc<Mutex<PgUserRepository>>,
    user_credential_repository: Arc<Mutex<PgUserCredentialRepository>>,
}

impl PgRepositories {
    pub fn new(tx: Transaction<'static, Postgres>) -> Self {
        let tx = Arc::new(Mutex::new(tx));
        let user_repository = Arc::new(Mutex::new(PgUserRepository::new(tx.clone())));
        let user_credential_repository =
            Arc::new(Mutex::new(PgUserCredentialRepository::new(tx.clone())));

        Self {
            tx,
            user_repository,
            user_credential_repository,
        }
    }
}

#[async_trait]
impl Repositories for PgRepositories {
    fn user_repository(&mut self) -> Arc<Mutex<dyn UserRepository>> {
        self.user_repository.clone()
    }

    fn user_credential_repository(&mut self) -> Arc<Mutex<dyn UserCredentialRepository>> {
        self.user_credential_repository.clone()
    }
}
