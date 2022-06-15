use thiserror::Error;
use todo_app_domain::error::RepositoryError;

#[derive(Debug, Error)]
#[error("usecase error")]
pub enum UsecaseError {
    Failed(&'static str),
    Repository(#[from] RepositoryError),
}
