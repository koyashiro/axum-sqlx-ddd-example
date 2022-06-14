use thiserror::Error;

#[derive(Debug, Error)]
#[error("session store error")]
pub struct SessionStoreError {
    #[source]
    source: Box<dyn std::error::Error>,
}

impl SessionStoreError {
    pub fn new(source: impl std::error::Error + 'static) -> Self {
        Self {
            source: Box::new(source),
        }
    }
}
