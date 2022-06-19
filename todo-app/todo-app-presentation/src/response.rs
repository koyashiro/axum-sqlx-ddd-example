use std::borrow::Cow;

use axum::{http::StatusCode, response::IntoResponse};
use serde::{Serialize, Serializer};

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    #[serde(serialize_with = "as_u16")]
    status_code: StatusCode,
    message: Cow<'static, str>,
    errors: Vec<ErrorDetail>,
}

impl ErrorResponse {
    pub fn bad_request(message: impl Into<Cow<'static, str>>, errors: Vec<ErrorDetail>) -> Self {
        Self {
            status_code: StatusCode::BAD_REQUEST,
            message: message.into(),
            errors,
        }
    }

    pub fn unauthorized() -> Self {
        Self {
            status_code: StatusCode::UNAUTHORIZED,
            message: "unauthorized".into(),
            errors: Default::default(),
        }
    }

    pub fn internal_server_error() -> Self {
        Self {
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
            message: "Internal server error".into(),
            errors: Default::default(),
        }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> axum::response::Response {
        (self.status_code, self).into_response()
    }
}

fn as_u16<S>(status_code: &StatusCode, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    s.serialize_u16(status_code.as_u16())
}

#[derive(Debug, Serialize)]
pub struct ErrorDetail {
    field: &'static str,
}
