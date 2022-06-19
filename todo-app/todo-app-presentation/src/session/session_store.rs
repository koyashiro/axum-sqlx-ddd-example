use async_trait::async_trait;
use chrono::{DateTime, Local};
use getset::Getters;
use serde::{Deserialize, Serialize};
use todo_app_domain::aggregate_root::user::value_object::UserId;
use uuid::Uuid;

pub const SESSION_ID_HEADER: &str = "_todo_app_session_id";

#[derive(Debug, Deserialize, Getters, Serialize)]
pub struct Session {
    #[getset(get = "pub")]
    user_id: Uuid,
    #[getset(get = "pub")]
    logged_in_at: DateTime<Local>,
}

impl Session {
    pub fn new(user_id: UserId) -> Self {
        Self {
            user_id: user_id.into_uuid(),
            logged_in_at: Local::now(),
        }
    }

    pub fn into_user_id(self) -> Uuid {
        self.user_id
    }
}

#[async_trait]
pub trait SessionStore: std::fmt::Debug + Send + Sync {
    async fn find(&self, session_id: &str) -> Result<Option<UserId>, anyhow::Error>;
    async fn save(&self, session_id: &str, session: &Session) -> Result<(), anyhow::Error>;
    async fn delete(&self, session_id: &str) -> Result<(), anyhow::Error>;
}
