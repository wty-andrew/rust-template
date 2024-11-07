use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;

pub enum ApiError {
    BadRequest(String),
    NotFound(String),
    InternalError(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        #[derive(Serialize)]
        struct ErrorResponse {
            message: String,
        }

        let (status, message) = match self {
            ApiError::BadRequest(message) => (StatusCode::BAD_REQUEST, message),
            ApiError::NotFound(message) => (StatusCode::NOT_FOUND, message),
            ApiError::InternalError(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}
