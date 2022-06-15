use async_trait::async_trait;
use redis::{AsyncCommands, Client};
use todo_app_domain::aggregate_root::user::value_object::UserId;
use todo_app_presentation::session::{error::SessionStoreError, Session, SessionStore};
use uuid::Uuid;

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
    async fn find(&self, session_id: &str) -> Result<Option<UserId>, SessionStoreError> {
        let mut conn = self
            .client
            .get_async_connection()
            .await
            .map_err(SessionStoreError::new)?;
        let session_string = conn
            .get::<&str, String>(session_id)
            .await
            .map_err(SessionStoreError::new)?;
        let session = Uuid::parse_str(&session_string).ok().map(UserId::from);

        Ok(session)
    }

    async fn save(&self, session_id: &str, session: &Session) -> Result<(), SessionStoreError> {
        let mut conn = self
            .client
            .get_async_connection()
            .await
            .map_err(SessionStoreError::new)?;
        let session_string = serde_json::to_string(&session).map_err(SessionStoreError::new)?;
        conn.set(session_id, &session_string)
            .await
            .map_err(SessionStoreError::new)?;

        Ok(())
    }

    async fn delete(&self, session_id: &str) -> Result<(), SessionStoreError> {
        let mut conn = self
            .client
            .get_async_connection()
            .await
            .map_err(SessionStoreError::new)?;
        conn.del(&session_id)
            .await
            .map_err(SessionStoreError::new)?;

        Ok(())
    }
}
