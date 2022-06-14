use async_trait::async_trait;
use redis::{AsyncCommands, Client, RedisError};

use todo_app_domain::aggregate_root::user::value_object::UserId;
use todo_app_presentation::session::{error::SessionStoreError, SessionId, SessionStore};

#[derive(Clone, Debug)]
pub struct RedisSessionStore {
    client: Client,
}

impl RedisSessionStore {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
}

#[async_trait]
impl SessionStore for RedisSessionStore {
    async fn find(&self, session_id: &SessionId) -> Result<Option<UserId>, SessionStoreError> {
        let mut conn = self
            .client
            .get_async_connection()
            .await
            .map_err(to_session_store_error)?;
        let user_id_raw = conn.get::<&str, String>(&session_id.as_str()).await.ok();
        let user_id = match user_id_raw {
            Some(user_id) => UserId::parse_str(&user_id).ok(),
            None => return Ok(None),
        };

        Ok(user_id)
    }

    async fn save(
        &self,
        session_id: &SessionId,
        user_id: &UserId,
    ) -> Result<(), SessionStoreError> {
        let mut conn = self
            .client
            .get_async_connection()
            .await
            .map_err(to_session_store_error)?;
        conn.set(
            session_id.as_str(),
            user_id.as_uuid().to_simple_ref().to_string(),
        )
        .await
        .map_err(to_session_store_error)?;

        Ok(())
    }

    async fn delete(&self, session_id: &SessionId) -> Result<(), SessionStoreError> {
        let mut conn = self
            .client
            .get_async_connection()
            .await
            .map_err(to_session_store_error)?;
        conn.del(&session_id.as_str())
            .await
            .map_err(to_session_store_error)?;

        Ok(())
    }
}

fn to_session_store_error(e: RedisError) -> SessionStoreError {
    SessionStoreError::new(e)
}
