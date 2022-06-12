use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

use crate::domain::user::{
    entity::{User, UserDto},
    repository::UserRepository,
    value_object::UserId,
};

#[derive(Debug)]
pub struct PgUserRepository {
    tx: Arc<Mutex<Transaction<'static, Postgres>>>,
}

impl PgUserRepository {
    pub fn new(tx: Arc<Mutex<Transaction<'static, Postgres>>>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl UserRepository for PgUserRepository {
    async fn find(&mut self, user_id: &UserId) -> Result<Option<User>, ()> {
        let mut tx = *self.tx.lock().unwrap();

        let user = sqlx::query_as!(
            UserDto,
            "
            SELECT id, name
            FROM users
            WHERE id = $1
            ",
            user_id.as_uuid()
        )
        .fetch_optional(&mut tx)
        .await
        .unwrap()
        .map(|u| User::try_from(u).unwrap());

        Ok(user)
    }

    async fn insert(&mut self, user: &User) -> Result<(), ()> {
        let mut tx = *self.tx.lock().unwrap();

        sqlx::query!(
            "
            INSERT INTO users (id, name)
            VALUES ($1, $2)
            ",
            user.id().as_uuid(),
            user.name().as_str()
        )
        .execute(&mut tx)
        .await
        .unwrap();

        Ok(())
    }

    async fn update(&mut self, user: &User) -> Result<(), ()> {
        let mut tx = *self.tx.lock().unwrap();

        sqlx::query!(
            "
            UPDATE users
            SET name = $1
            WHERE id = $2
            ",
            user.name().as_str(),
            user.id().as_uuid(),
        )
        .execute(&mut tx)
        .await
        .unwrap();

        Ok(())
    }

    async fn delete(&mut self, user_id: &UserId) -> Result<(), ()> {
        let mut tx = *self.tx.lock().unwrap();

        sqlx::query!(
            "
            DELETE FROM users
            WHERE id = $1
            ",
            user_id.as_uuid(),
        )
        .execute(&mut tx)
        .await
        .unwrap();

        Ok(())
    }
}
