use async_trait::async_trait;

use crate::database::Repositories;

#[async_trait]
pub trait Transaction: Repositories {
    async fn commit(self: Box<Self>);
    async fn rollback(self: Box<Self>);
}
