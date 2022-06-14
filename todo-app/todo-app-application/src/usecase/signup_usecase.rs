use std::sync::Arc;

use crate::database::DB;

#[derive(Clone, Debug)]
pub struct SignupUsecase {
    db: Arc<dyn DB>,
}

impl SignupUsecase {
    pub fn new(db: Arc<dyn DB>) -> Self {
        Self { db }
    }

    pub async fn execute(
        &self,
        _name: String,
        _email: String,
        _password: String,
    ) -> Result<(), ()> {
        Ok(())
    }
}
