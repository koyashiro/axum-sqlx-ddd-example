use sqlx::{PgPool, Postgres, Transaction};

#[derive(Debug)]
pub enum Connection {
    Pool(PgPool),
    Transaction(Transaction<'static, Postgres>),
}

impl Into<Connection> for Transaction<'static, Postgres> {
    fn into(self) -> Connection {
        Connection::Transaction(self)
    }
}

impl Into<Connection> for PgPool {
    fn into(self) -> Connection {
        Connection::Pool(self)
    }
}
