use axum::body::Body;
use axum::http::{HeaderValue, Request};
use axum::{routing::get, Router};
use sqlx::PgPool;
use tower::ServiceBuilder;
use tower_http::request_id::{MakeRequestId, RequestId};
use tower_http::trace::TraceLayer;
use tower_http::ServiceBuilderExt;
use uuid::Uuid;

use crate::routes::{liveness, not_found, readiness, todo};

#[derive(Clone, Copy)]
struct MakeRequestUuid;

impl MakeRequestId for MakeRequestUuid {
    fn make_request_id<B>(&mut self, _request: &Request<B>) -> Option<RequestId> {
        match Uuid::new_v4().to_string().parse() {
            Ok(request_id) => Some(RequestId::new(request_id)),
            Err(_) => None,
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub db_pool: PgPool,
}

pub fn create_app(db_pool: PgPool) -> Router {
    let state = AppState { db_pool };

    let middleware = ServiceBuilder::new()
        .set_x_request_id(MakeRequestUuid)
        .layer(
            TraceLayer::new_for_http().make_span_with(|req: &Request<Body>| {
                let request_id = req
                    .headers()
                    .get("x-request-id")
                    .unwrap_or(&HeaderValue::from_static(""))
                    .to_str()
                    .unwrap_or_default()
                    .to_string();
                tracing::info_span!(
                    "request",
                    request_id,
                    method = %req.method(),
                    uri = %req.uri()
                )
            }),
        )
        .propagate_x_request_id();

    Router::new()
        .nest("/todos", todo::router())
        .layer(middleware)
        .route("/healthz", get(liveness))
        .route("/ready", get(readiness))
        .fallback(not_found)
        .with_state(state)
}
