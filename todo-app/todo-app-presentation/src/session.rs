mod session_id;
mod session_store;

pub mod error;

pub use session_id::SessionId;
pub use session_store::SessionStore;

pub const SESSION_ID_HEADER: &str = "_todo_app_session_id";
