use axum::response::IntoResponse;
use thiserror::Error;
use todo_app_application::usecase::error::UsecaseError;

use crate::response::ErrorResponse;

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("HandlerError::Usecase: {0:?}")]
    Usecase(#[from] UsecaseError),
    #[error("Unauthorized")]
    Authentication,
    #[error("Internal server error")]
    Unexpected(#[from] anyhow::Error),
}

impl IntoResponse for HandlerError {
    fn into_response(self) -> axum::response::Response {
        match self {
            Self::Usecase(e) => ErrorResponse::bad_request(e.to_string(), Default::default()),
            Self::Authentication => ErrorResponse::unauthorized(),
            Self::Unexpected(e) => {
                tracing::error!("{e:?}");
                ErrorResponse::internal_server_error()
            }
        }
        .into_response()
    }
}
