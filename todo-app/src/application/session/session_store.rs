use std::fmt::Debug;

use async_trait::async_trait;

use crate::{application::session::SessionId, domain::aggregate_root::user::value_object::UserId};

#[async_trait]
pub trait SessionStore: Debug + Send + Sync {
    async fn find(&self, session_id: &SessionId) -> Result<Option<UserId>, anyhow::Error>;
    async fn save(&self, session_id: &SessionId, user_id: &UserId) -> Result<(), anyhow::Error>;
    async fn delete(&self, session_id: &SessionId) -> Result<(), anyhow::Error>;
}
