use std::sync::Arc;

use sqlx::{PgPool, Postgres, Transaction};
use tokio::sync::Mutex;

#[derive(Clone, Debug)]
pub enum PgConnection {
    Pool(PgPool),
    Transaction(Arc<Mutex<Transaction<'static, Postgres>>>),
}

impl From<PgPool> for PgConnection {
    fn from(pool: PgPool) -> Self {
        PgConnection::Pool(pool)
    }
}

impl From<Arc<Mutex<Transaction<'static, Postgres>>>> for PgConnection {
    fn from(tx: Arc<Mutex<Transaction<'static, Postgres>>>) -> Self {
        PgConnection::Transaction(tx)
    }
}
