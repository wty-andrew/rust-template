use axum::http::{Request, StatusCode};
use sqlx::PgPool;
use tower::ServiceExt;

use sandbox::app::create_app;

mod common;
use common::utils::RequestBuilderExt;

#[sqlx::test]
async fn liveness_probe_returns_200(pool: PgPool) {
    let app = create_app(pool);

    let request = Request::get("/healthz").empty_body();
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[sqlx::test]
async fn readiness_probe_returns_200(pool: PgPool) {
    let app = create_app(pool);

    let request = Request::get("/ready").empty_body();
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
