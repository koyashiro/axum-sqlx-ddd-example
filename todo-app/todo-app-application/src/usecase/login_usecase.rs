use std::sync::Arc;

use todo_app_domain::aggregate_root::user::value_object::UserId;

use crate::{database::DB, usecase::error::UsecaseError};

#[derive(Clone, Debug)]
pub struct LoginUsecase {
    db: Arc<dyn DB>,
}

impl LoginUsecase {
    pub fn new(db: Arc<dyn DB>) -> Self {
        Self { db }
    }

    pub async fn execute(&self, email: &str, password: &str) -> Result<UserId, UsecaseError> {
        let user_credential = self
            .db
            .user_credential_repository()
            .find_by_email(email)
            .await?
            .ok_or(login_failed_error())?;

        if !user_credential.password_hash().verify(password) {
            return Err(login_failed_error());
        }

        let (user_id, _, _) = user_credential.into_inner();

        Ok(user_id)
    }
}

fn login_failed_error() -> UsecaseError {
    UsecaseError::Expected {
        message: "invalid email or password",
        errors: Default::default(),
    }
}
