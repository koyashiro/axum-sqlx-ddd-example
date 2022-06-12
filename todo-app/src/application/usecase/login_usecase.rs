use std::sync::{Arc, Mutex};

use crate::application::dependency::Repositories;

#[derive(Debug)]
pub struct LoginUsecase {
    repositories: Arc<Mutex<dyn Repositories>>,
}

impl LoginUsecase {
    pub fn new(repositories: Arc<Mutex<dyn Repositories>>) -> Self {
        Self { repositories }
    }

    pub async fn execute(&mut self, email: String, password: String) -> Result<(), ()> {
        let email = email.try_into().unwrap();

        let user_credential = self
            .repositories
            .lock()
            .unwrap()
            .user_credential_repository()
            .lock()
            .unwrap()
            .find_by_email(&email)
            .await
            .unwrap()
            .unwrap();

        if !user_credential.password_hash().verify(&password) {
            panic!("error");
        }

        Ok(())
    }
}
