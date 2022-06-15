use thiserror::Error;

#[derive(Debug, Error)]
#[error("repository error")]
pub struct RepositoryError(Box<dyn std::error::Error>);

impl RepositoryError {
    pub fn from(e: impl std::error::Error + 'static) -> Self {
        Self(Box::new(e))
    }
}
