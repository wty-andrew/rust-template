use axum::http::StatusCode;

pub async fn not_found() -> StatusCode {
    StatusCode::NOT_FOUND
}
