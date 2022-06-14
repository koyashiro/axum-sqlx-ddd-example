use std::sync::Arc;

use axum::{response::IntoResponse, Extension, Json};
use serde::{Deserialize, Serialize};

use todo_app_application::usecase::LoginUsecase;
use tower_cookies::{Cookie, Cookies};

use crate::session::{SessionId, SessionStore, SESSION_ID_HEADER};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    message: &'static str,
}

pub async fn login(
    cookies: Cookies,
    Json(request): Json<LoginRequest>,
    Extension(login_usecase): Extension<LoginUsecase>,
    Extension(session_store): Extension<Arc<dyn SessionStore>>,
) -> impl IntoResponse {
    let user_id = login_usecase
        .execute(&request.email, &request.password)
        .await
        .unwrap();

    let session_id = SessionId::new();
    session_store.save(&session_id, &user_id).await.unwrap();

    let cookie = Cookie::build(
        SESSION_ID_HEADER,
        user_id.as_uuid().to_simple_ref().to_string(),
    )
    .finish();
    cookies.add(cookie);

    Json(LoginResponse { message: "ok" })
}
