use std::sync::Arc;

use async_trait::async_trait;

use crate::domain::database::{Repositories, Transaction};

#[async_trait]
pub trait DB: Repositories {
    async fn begin(&self) -> Arc<dyn Transaction>;
}
