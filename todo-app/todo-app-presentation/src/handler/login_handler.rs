use std::sync::Arc;

use axum::{Extension, Json};
use cookie::{time::OffsetDateTime, SameSite};
use serde::{Deserialize, Serialize};
use time::Duration;
use todo_app_application::usecase::LoginUsecase;
use tower_cookies::{Cookie, Cookies};
use uuid::Uuid;

use crate::{
    handler::error::HandlerError,
    session::{Session, SessionStore, SESSION_ID_HEADER},
};

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Debug, Default, Serialize)]
pub struct LoginResponse {
    message: &'static str,
}

pub async fn login(
    cookies: Cookies,
    Json(request): Json<LoginRequest>,
    Extension(login_usecase): Extension<LoginUsecase>,
    Extension(session_store): Extension<Arc<dyn SessionStore>>,
) -> Result<Json<LoginResponse>, HandlerError> {
    let user_id = login_usecase
        .execute(&request.email, &request.password)
        .await?;

    let session_id = format!("{}{}", Uuid::new_v4().simple(), Uuid::new_v4().simple());
    let session = Session::new(user_id);
    session_store.save(&session_id, &session).await?;

    let cookie = Cookie::build(SESSION_ID_HEADER, session_id)
        .expires(OffsetDateTime::now_utc() + Duration::days(30))
        .http_only(true)
        .same_site(SameSite::Strict)
        .finish();
    cookies.add(cookie);

    Ok(Json(LoginResponse { message: "ok" }))
}
