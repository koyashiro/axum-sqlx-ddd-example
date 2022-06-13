use async_trait::async_trait;
use redis::{AsyncCommands, Client};

use crate::{
    application::session::{SessionId, SessionStore},
    domain::aggregate_root::user::value_object::UserId,
};

#[derive(Debug)]
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
    async fn find(&self, session_id: &SessionId) -> Result<Option<UserId>, anyhow::Error> {
        let mut conn = self.client.get_async_connection().await?;
        let user_id_raw = conn
            .get::<&str, String>(&session_id.to_simple_string())
            .await?;
        let user_id = user_id_raw.parse()?;

        Ok(Some(user_id))
    }

    async fn save(&self, session_id: &SessionId, user_id: &UserId) -> Result<(), anyhow::Error> {
        let mut conn = self.client.get_async_connection().await?;
        conn.set(
            &session_id.to_simple_string(),
            user_id.as_uuid().to_simple_ref().to_string(),
        )
        .await?;

        Ok(())
    }

    async fn delete(&self, session_id: &SessionId) -> Result<(), anyhow::Error> {
        let mut conn = self.client.get_async_connection().await?;
        conn.del(&session_id.to_simple_string()).await?;

        Ok(())
    }
}
