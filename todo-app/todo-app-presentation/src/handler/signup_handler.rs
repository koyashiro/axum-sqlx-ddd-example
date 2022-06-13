use axum::{response::IntoResponse, Extension};

use todo_app_application::usecase::SignupUsecase;

pub async fn signup(Extension(signup_usecase): Extension<SignupUsecase>) -> impl IntoResponse {
    signup_usecase
        .execute(
            "user name".to_owned(),
            "user@example.com".to_owned(),
            "password".to_owned(),
        )
        .await
        .unwrap();

    "Hello, world!!!"
}
