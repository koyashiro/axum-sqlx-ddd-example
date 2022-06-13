use std::sync::Arc;

use axum::{
    http::{HeaderMap, StatusCode},
    response::IntoResponse,
    Extension, Json,
};
use serde::{Deserialize, Serialize};

use todo_app_application::usecase::LoginUsecase;

use crate::session::{SessionId, SessionStore};

const SESSION_ID: &str = "_session_id";

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

    let mut headers = HeaderMap::new();
    headers.insert(SESSION_ID, user_id.to_string().as_str().parse().unwrap());

    (
        StatusCode::OK,
        headers,
        Json(LoginResponse { message: "ok" }),
    )
}
