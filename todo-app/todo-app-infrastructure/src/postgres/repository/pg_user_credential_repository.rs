use async_trait::async_trait;

use todo_app_domain::aggregate_root::{
    user::value_object::UserId,
    user_credential::{
        entity::{UserCredential, UserCredentialHashedRaw},
        repository::UserCredentialRepository,
        value_object::Email,
    },
};

use crate::postgres::database::PgConnection;

#[derive(Debug)]
pub struct PgUserCredentialRepository {
    conn: PgConnection,
}

impl<'a> PgUserCredentialRepository {
    pub fn new(conn: PgConnection) -> Self {
        Self { conn }
    }
}

#[async_trait]
impl<'a> UserCredentialRepository for PgUserCredentialRepository {
    async fn find(&self, user_id: &UserId) -> Result<Option<UserCredential>, ()> {
        let query = sqlx::query_as!(
            UserCredentialHashedRaw,
            "
            SELECT user_id, email, password_hash
            FROM user_credentials
            WHERE user_id = $1
            ",
            user_id.as_uuid()
        );

        let user = match &self.conn {
            PgConnection::Pool(p) => query.fetch_optional(p).await,
            PgConnection::Transaction(tx) => query.fetch_optional(&mut *tx.lock().await).await,
        }
        .unwrap()
        .map(|uc| UserCredential::try_from(uc).unwrap());

        Ok(user)
    }

    async fn find_by_email(&self, email: &Email) -> Result<Option<UserCredential>, ()> {
        let query = sqlx::query_as!(
            UserCredentialHashedRaw,
            "
            SELECT user_id, email, password_hash
            FROM user_credentials
            WHERE email = $1
            ",
            email.as_str()
        );

        let user = match &self.conn {
            PgConnection::Pool(p) => query.fetch_optional(p).await,
            PgConnection::Transaction(tx) => query.fetch_optional(&mut *tx.lock().await).await,
        }
        .unwrap()
        .map(|uc| UserCredential::try_from(uc).unwrap());

        Ok(user)
    }

    async fn insert(&self, user_credential: &UserCredential) -> Result<(), ()> {
        let query = sqlx::query!(
            "
            INSERT INTO user_credentials (user_id, email, password_hash)
            VALUES ($1, $2, $3)
            ",
            user_credential.user_id().as_uuid(),
            user_credential.email().as_str(),
            user_credential.password_hash().as_str()
        );

        match &self.conn {
            PgConnection::Pool(p) => query.execute(p).await,
            PgConnection::Transaction(tx) => query.execute(&mut *tx.lock().await).await,
        }
        .unwrap();

        Ok(())
    }

    async fn update(&self, user_credential: &UserCredential) -> Result<(), ()> {
        let query = sqlx::query!(
            "
            UPDATE user_credentials
            SET email = $1, password_hash = $2
            WHERE user_id = $3
            ",
            user_credential.email().as_str(),
            user_credential.password_hash().as_str(),
            user_credential.user_id().as_uuid(),
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
            DELETE FROM user_credentials
            WHERE user_id = $1
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