use axum::response::IntoResponse;
use thiserror::Error;
use todo_app_application::usecase::error::UsecaseError;
use todo_app_domain::error::RepositoryError;

use crate::session::error::SessionStoreError;

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("usecase error")]
    Usecase(#[from] UsecaseError),
    #[error("session store")]
    SessionStore(#[from] SessionStoreError),
    #[error("repository error")]
    Repository(#[from] RepositoryError),
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> axum::response::Response {
        todo!()
    }
}
