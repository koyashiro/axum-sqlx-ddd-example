use std::sync::Arc;

use crate::domain::{
    user::{
        entity::User,
        repository::UserRepository,
        value_object::{UserId, UserName},
    },
    user_credential::{
        entity::UserCredential,
        repository::UserCredentialRepository,
        value_object::{Email, Password},
    },
};

#[derive(Debug)]
pub struct SignupUsecaseArgs {
    pub name: UserName,
    pub email: Email,
    pub password: Password,
}

#[derive(Debug)]
pub struct SignupUsecase {
    user_repository: Arc<dyn UserRepository>,
    user_credential_repository: Arc<dyn UserCredentialRepository>,
}

impl SignupUsecase {
    pub async fn execute(&self, args: SignupUsecaseArgs) -> Result<(), ()> {
        let user = User::new(UserId::new(), args.name);
        self.user_repository.insert(&user).await?;

        let credential =
            UserCredential::new(user.id().to_owned(), args.email, args.password.to_hash());
        self.user_credential_repository.insert(&credential).await?;

        Ok(())
    }
}
