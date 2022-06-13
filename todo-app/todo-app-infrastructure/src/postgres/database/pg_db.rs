use std::sync::Arc;

use async_trait::async_trait;
use sqlx::PgPool as SqlxPgPool;

use todo_app_application::database::{Repositories, Transaction, DB};
use todo_app_domain::aggregate_root::{
    todo::repository::TodoRepository, user::repository::UserRepository,
    user_credential::repository::UserCredentialRepository,
};

use crate::postgres::{
    database::{PgConnection, PgTransaction},
    repository::{PgUserCredentialRepository, PgUserRepository},
};

#[derive(Clone, Debug)]
pub struct PgDB {
    pool: SqlxPgPool,
}

impl PgDB {
    pub fn new(pool: SqlxPgPool) -> Self {
        Self { pool }
    }
}

impl Repositories for PgDB {
    fn user_repository(&self) -> Arc<dyn UserRepository> {
        Arc::new(PgUserRepository::new(PgConnection::Pool(self.pool.clone())))
    }

    fn user_credential_repository(&self) -> Arc<dyn UserCredentialRepository> {
        Arc::new(PgUserCredentialRepository::new(PgConnection::Pool(
            self.pool.clone(),
        )))
    }

    fn todo_repository(&self) -> Arc<dyn TodoRepository> {
        // Arc::new(PgTodoRepository::new(self.conn.clone()))
        todo!()
    }
}

#[async_trait]
impl DB for PgDB {
    async fn begin(&self) -> Arc<dyn Transaction> {
        Arc::new(PgTransaction::new(self.pool.begin().await.unwrap()))
    }
}
