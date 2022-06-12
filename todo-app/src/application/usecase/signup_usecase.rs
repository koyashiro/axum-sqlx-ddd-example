use std::sync::{Arc, Mutex};

use uuid::Uuid;

use crate::{
    application::dependency::Repositories,
    domain::{
        user::entity::{User, UserDto},
        user_credential::entity::{UserCredential, UserCredentialDto},
    },
};

#[derive(Debug)]
pub struct SignupUsecase {
    repositories: Arc<Mutex<dyn Repositories>>,
}

impl SignupUsecase {
    pub fn new(repositories: Arc<Mutex<dyn Repositories>>) -> Self {
        Self { repositories }
    }

    pub async fn execute(
        &mut self,
        name: String,
        email: String,
        password: String,
    ) -> Result<(), ()> {
        let user_id = Uuid::new_v4();

        let user: User = UserDto { id: user_id, name }.try_into().unwrap();

        self.repositories
            .lock()
            .unwrap()
            .user_repository()
            .lock()
            .unwrap()
            .insert(&user)
            .await?;

        let credential: UserCredential = UserCredentialDto {
            user_id,
            email,
            password,
        }
        .try_into()
        .unwrap();

        self.repositories
            .lock()
            .unwrap()
            .user_credential_repository()
            .lock()
            .unwrap()
            .insert(&credential)
            .await
            .unwrap();

        Ok(())
    }
}
