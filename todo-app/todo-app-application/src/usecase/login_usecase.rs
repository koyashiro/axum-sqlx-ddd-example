use std::sync::Arc;

use todo_app_domain::aggregate_root::user::value_object::UserId;

use crate::database::DB;

#[derive(Clone, Debug)]
pub struct LoginUsecase {
    db: Arc<dyn DB>,
}

impl LoginUsecase {
    pub fn new(db: Arc<dyn DB>) -> Self {
        Self { db }
    }

    pub async fn execute(&self, email: &str, password: &str) -> Result<UserId, ()> {
        let user_credential = self
            .db
            .user_credential_repository()
            .find_by_email(email)
            .await
            .unwrap()
            .unwrap();

        if !user_credential.password_hash().verify(password) {
            todo!()
        }

        Ok(user_credential.user_id().clone())
    }
}
