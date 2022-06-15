use async_trait::async_trait;

use nameof::name_of;
use todo_app_domain::{
    aggregate_root::user::{
        entity::User,
        repository::UserRepository,
        value_object::{UserId, UserName},
    },
    error::{RepositoryError, ValidationErrors},
};
use uuid::Uuid;

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
    async fn find(&self, user_id: &UserId) -> Result<Option<User>, RepositoryError> {
        let query = sqlx::query_as!(
            UserRecord,
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
        .map_err(RepositoryError::from)?;

        let user = match user {
            Some(u) => User::try_from(u),
            None => return Ok(None),
        }
        .map_err(RepositoryError::from)?;

        Ok(Some(user))
    }

    async fn insert(&self, user: &User) -> Result<(), RepositoryError> {
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
        .map_err(RepositoryError::from)?;

        Ok(())
    }

    async fn update(&self, user: &User) -> Result<(), RepositoryError> {
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
        .map_err(RepositoryError::from)?;

        Ok(())
    }

    async fn delete(&self, user_id: &UserId) -> Result<(), RepositoryError> {
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
        .map_err(RepositoryError::from)?;

        Ok(())
    }
}

struct UserRecord {
    id: Uuid,
    name: String,
}

impl TryFrom<UserRecord> for User {
    type Error = ValidationErrors;

    fn try_from(value: UserRecord) -> Result<Self, Self::Error> {
        let id = UserId::from(value.id);
        let name = UserName::try_from(value.name);
        match name {
            Ok(name) => Ok(User::from((id, name))),
            Err(name) => {
                let mut errors = Self::Error::new();
                errors.add(name_of!(name), name);
                Err(errors)
            }
        }
    }
}
