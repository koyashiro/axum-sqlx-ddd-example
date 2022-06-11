use std::sync::Arc;

use crate::domain::user::{
    entity::User,
    repository::UserRepository,
    value_object::{UserId, UserName},
};

#[derive(Debug)]
pub struct ChangeUserNameUsecaseArgs {
    pub id: UserId,
    pub name: UserName,
}

#[derive(Debug)]
pub struct ChangeUserNameUsecase {
    user_repository: Arc<dyn UserRepository>,
}

impl ChangeUserNameUsecase {
    pub async fn execute(&self, args: ChangeUserNameUsecaseArgs) -> Result<User, ()> {
        let mut user = self.user_repository.find(&args.id).await?.ok_or(())?;
        user.set_name(args.name);
        self.user_repository.update(&user).await?;

        Ok(user)
    }
}
