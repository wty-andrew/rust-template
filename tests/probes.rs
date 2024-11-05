use axum::body::Body;
use axum::http::{Method, Request, StatusCode};
use tower::ServiceExt;

use sandbox::app::create_app;
use sandbox::settings::Settings;

#[tokio::test]
async fn liveness_probe_returns_200() {
    let app = create_app(Settings::new().unwrap()).await.unwrap();

    let request = Request::builder()
        .method(Method::GET)
        .uri("/healthz")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn readiness_probe_returns_200() {
    let app = create_app(Settings::new().unwrap()).await.unwrap();

    let request = Request::builder()
        .method(Method::GET)
        .uri("/ready")
        .body(Body::empty())
        .unwrap();
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
}
