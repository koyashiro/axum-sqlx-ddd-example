use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use sqlx::{Postgres, Transaction};

use crate::domain::{
    user::value_object::UserId,
    user_credential::{
        entity::{UserCredential, UserCredentialHashedDto},
        repository::UserCredentialRepository,
        value_object::Email,
    },
};

#[derive(Debug)]
pub struct PgUserCredentialRepository {
    tx: Arc<Mutex<Transaction<'static, Postgres>>>,
}

impl<'a> PgUserCredentialRepository {
    pub fn new(tx: Arc<Mutex<Transaction<'static, Postgres>>>) -> Self {
        Self { tx }
    }
}

#[async_trait]
impl<'a> UserCredentialRepository for PgUserCredentialRepository {
    async fn find(&mut self, user_id: &UserId) -> Result<Option<UserCredential>, ()> {
        let mut tx = *self.tx.lock().unwrap();

        let user = sqlx::query_as!(
            UserCredentialHashedDto,
            "
            SELECT user_id, email, password_hash
            FROM user_credentials
            WHERE user_id = $1
            ",
            user_id.as_uuid()
        )
        .fetch_optional(&mut tx)
        .await
        .unwrap()
        .map(|u| UserCredential::try_from(u).unwrap());

        Ok(user)
    }

    async fn find_by_email(&mut self, email: &Email) -> Result<Option<UserCredential>, ()> {
        let mut tx = *self.tx.lock().unwrap();

        let user = sqlx::query_as!(
            UserCredentialHashedDto,
            "
            SELECT user_id, email, password_hash
            FROM user_credentials
            WHERE email = $1
            ",
            email.as_str()
        )
        .fetch_optional(&mut tx)
        .await
        .unwrap()
        .map(|u| UserCredential::try_from(u).unwrap());

        Ok(user)
    }

    async fn insert(&mut self, user_credential: &UserCredential) -> Result<(), ()> {
        let mut tx = *self.tx.lock().unwrap();

        sqlx::query!(
            "
            INSERT INTO user_credentials (user_id, email, password_hash)
            VALUES ($1, $2, $3)
            ",
            user_credential.user_id().as_uuid(),
            user_credential.email().as_str(),
            user_credential.password_hash().as_str()
        )
        .execute(&mut tx)
        .await
        .unwrap();

        Ok(())
    }

    async fn update(&mut self, user_credential: &UserCredential) -> Result<(), ()> {
        let mut tx = *self.tx.lock().unwrap();

        sqlx::query!(
            "
            UPDATE user_credentials
            SET email = $1, password_hash = $2
            WHERE user_id = $3
            ",
            user_credential.email().as_str(),
            user_credential.password_hash().as_str(),
            user_credential.user_id().as_uuid(),
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
            DELETE FROM user_credentials
            WHERE user_id = $1
            ",
            user_id.as_uuid(),
        )
        .execute(&mut tx)
        .await
        .unwrap();

        Ok(())
    }
}
