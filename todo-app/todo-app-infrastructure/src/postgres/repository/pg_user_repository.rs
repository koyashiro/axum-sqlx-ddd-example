use async_trait::async_trait;

use todo_app_domain::aggregate_root::user::{
    entity::{User, UserRaw},
    repository::UserRepository,
    value_object::UserId,
};

use crate::postgres::database::PgConnection;

#[derive(Debug)]
pub struct PgUserRepository {
    conn: PgConnection,
}

impl PgUserRepository {
    pub fn new(conn: PgConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find(&self, user_id: &UserId) -> Result<Option<User>, ()> {
        let query = sqlx::query_as!(
            UserRaw,
            "
            SELECT id, name
            FROM users
            WHERE id = $1
            ",
            user_id.as_uuid()
        );

        let user = match &self.conn {
            PgConnection::Pool(p) => query.fetch_optional(p).await,
            PgConnection::Transaction(tx) => query.fetch_optional(&mut *tx.lock().await).await,
        }
        .unwrap()
        .map(|u| User::try_from(u).unwrap());

        Ok(user)
    }

    async fn insert(&self, user: &User) -> Result<(), ()> {
        let query = sqlx::query!(
            "
            INSERT INTO users (id, name)
            VALUES ($1, $2)
            ",
            user.id().as_uuid(),
            user.name().as_str()
        );

        match &self.conn {
            PgConnection::Pool(p) => query.execute(p).await,
            PgConnection::Transaction(tx) => query.execute(&mut *tx.lock().await).await,
        }
        .unwrap();

        Ok(())
    }

    async fn update(&self, user: &User) -> Result<(), ()> {
        let query = sqlx::query!(
            "
            UPDATE users
            SET name = $1
            WHERE id = $2
            ",
            user.name().as_str(),
            user.id().as_uuid(),
        );

        match &self.conn {
            PgConnection::Pool(p) => query.execute(p).await,
            PgConnection::Transaction(tx) => query.execute(&mut *tx.lock().await).await,
        }
        .unwrap();

        Ok(())
    }

    async fn delete(&self, user_id: &UserId) -> Result<(), ()> {
        let query = sqlx::query!(
            "
            DELETE FROM users
            WHERE id = $1
            ",
            user_id.as_uuid(),
        );

        match &self.conn {
            PgConnection::Pool(p) => query.execute(p).await,
            PgConnection::Transaction(tx) => query.execute(&mut *tx.lock().await).await,
        }
        .unwrap();

        Ok(())
    }
}
