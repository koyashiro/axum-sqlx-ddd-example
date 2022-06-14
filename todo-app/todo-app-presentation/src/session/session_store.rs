use std::fmt::Debug;

use async_trait::async_trait;

use todo_app_domain::aggregate_root::user::value_object::UserId;

use crate::session::SessionId;

use super::error::SessionStoreError;

#[async_trait]
pub trait SessionStore: Debug + Send + Sync {
    async fn find(&self, session_id: &SessionId) -> Result<Option<UserId>, SessionStoreError>;
    async fn save(&self, session_id: &SessionId, user_id: &UserId)
        -> Result<(), SessionStoreError>;
    async fn delete(&self, session_id: &SessionId) -> Result<(), SessionStoreError>;
}
