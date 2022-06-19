use thiserror::Error;
use todo_app_domain::error::ValidationErrors;

#[derive(Debug, Error)]
pub enum UsecaseError {
    #[error("UsecaseError::Expected: {message}")]
    Expected {
        message: &'static str,
        errors: ValidationErrors,
    },
    #[error("UsecaseError::Unexpected: {0:?}")]
    Unexpected(#[from] anyhow::Error),
}
