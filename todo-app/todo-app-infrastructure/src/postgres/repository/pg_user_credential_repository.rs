use async_trait::async_trait;

use nameof::name_of;
use todo_app_domain::{
    aggregate_root::{
        user::value_object::UserId,
        user_credential::{
            entity::UserCredential,
            repository::UserCredentialRepository,
            value_object::{Email, PasswordHash},
        },
    },
    error::ValidationErrors,
};
use uuid::Uuid;

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
    async fn find(&self, user_id: &UserId) -> Result<Option<UserCredential>, anyhow::Error> {
        let query = sqlx::query_as!(
            UserCredentialRecord,
            "
            SELECT user_id, email, password_hash
            FROM user_credentials
            WHERE user_id = $1
            ",
            user_id.as_uuid()
        );

        let user_credential = match &self.conn {
            PgConnection::Pool(p) => query.fetch_optional(p).await,
            PgConnection::Transaction(tx) => query.fetch_optional(&mut *tx.lock().await).await,
        }?;

        let user_credential = match user_credential {
            Some(uc) => UserCredential::try_from(uc),
            None => return Ok(None),
        }?;

        Ok(Some(user_credential))
    }

    async fn find_by_email(&self, email: &str) -> Result<Option<UserCredential>, anyhow::Error> {
        let query = sqlx::query_as!(
            UserCredentialRecord,
            "
            SELECT user_id, email, password_hash
            FROM user_credentials
            WHERE email = $1
            ",
            email
        );

        let user = match &self.conn {
            PgConnection::Pool(p) => query.fetch_optional(p).await,
            PgConnection::Transaction(tx) => query.fetch_optional(&mut *tx.lock().await).await,
        }?;

        let user = match user {
            Some(uc) => UserCredential::try_from(uc),
            None => return Ok(None),
        }?;

        Ok(Some(user))
    }

    async fn insert(&self, user_credential: &UserCredential) -> Result<(), anyhow::Error> {
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
        }?;

        Ok(())
    }

    async fn update(&self, user_credential: &UserCredential) -> Result<(), anyhow::Error> {
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
        }?;

        Ok(())
    }

    async fn delete(&self, user_id: &UserId) -> Result<(), anyhow::Error> {
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
        }?;

        Ok(())
    }
}

#[derive(Debug)]
struct UserCredentialRecord {
    user_id: Uuid,
    email: String,
    password_hash: String,
}

impl TryFrom<UserCredentialRecord> for UserCredential {
    type Error = ValidationErrors;

    fn try_from(value: UserCredentialRecord) -> Result<Self, Self::Error> {
        let user_id = UserId::from(value.user_id);
        let email = Email::try_from(value.email);
        let password_hash = PasswordHash::try_from(value.password_hash);
        match (email, password_hash) {
            (Ok(email), Ok(password_hash)) => {
                Ok(UserCredential::from((user_id, email, password_hash)))
            }
            (email, password_hash) => Self::Error::builder()
                .result(name_of!(email), email)
                .result(name_of!(password_hash), password_hash)
                .build()
                .into(),
        }
    }
}
