use std::sync::Arc;

use todo_app_domain::database::DB;

#[derive(Clone, Debug)]
pub struct SignupUsecase {
    db: Arc<dyn DB>,
}

impl SignupUsecase {
    pub fn new(db: Arc<dyn DB>) -> Self {
        Self { db }
    }

    pub async fn execute(&self, name: String, email: String, password: String) -> Result<(), ()> {
        Ok(())
    }
}
