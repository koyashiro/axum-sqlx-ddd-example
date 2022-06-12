use std::sync::{Arc, Mutex};

use axum::{response::IntoResponse, Extension};
use sqlx::PgPool;

use crate::{application::usecase::LoginUsecase, infrastructure::dependency::PgRepositories};

pub async fn login(Extension(pool): Extension<PgPool>) -> impl IntoResponse {
    let tx = pool.begin().await.unwrap();
    let repositories = Arc::new(Mutex::new(PgRepositories::new(tx)));
    let mut login_usecase = LoginUsecase::new(repositories);
    login_usecase
        .execute("user@example.com".to_owned(), "password".to_owned())
        .await
        .unwrap();

    "Ok"
}
