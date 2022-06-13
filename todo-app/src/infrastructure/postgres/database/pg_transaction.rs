use std::sync::Arc;

use async_trait::async_trait;
use sqlx::{Postgres, Transaction as SqlxTransaction};
use tokio::sync::Mutex;

use crate::{
    domain::{
        aggregate_root::{
            todo::repository::TodoRepository, user::repository::UserRepository,
            user_credential::repository::UserCredentialRepository,
        },
        database::{Repositories, Transaction},
    },
    infrastructure::postgres::repository::{PgUserCredentialRepository, PgUserRepository},
};

#[derive(Debug)]
pub struct PgTransaction {
    tx: Arc<Mutex<SqlxTransaction<'static, Postgres>>>,
}

impl PgTransaction {
    pub fn new(tx: SqlxTransaction<'static, Postgres>) -> Self {
        Self {
            tx: Arc::new(Mutex::new(tx)),
        }
    }
}

impl Repositories for PgTransaction {
    fn user_repository(&self) -> Arc<dyn UserRepository> {
        Arc::new(PgUserRepository::new(self.tx.clone().into()))
    }

    fn user_credential_repository(&self) -> Arc<dyn UserCredentialRepository> {
        Arc::new(PgUserCredentialRepository::new(self.tx.clone().into()))
    }

    fn todo_repository(&self) -> Arc<dyn TodoRepository> {
        // Arc::new(PgTodoRepository::new(self.conn.clone()))
        todo!()
    }
}

#[async_trait]
impl Transaction for PgTransaction {
    async fn commit(self: Box<Self>) {
        self.commit().await;
    }

    async fn rollback(self: Box<Self>) {
        self.rollback().await;
    }
}
