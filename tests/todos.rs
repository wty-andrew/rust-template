use std::borrow::BorrowMut;

use axum::body::Body;
use axum::http::{Request, Response, StatusCode};
use serde_json::json;
use sqlx::PgPool;
use tower::ServiceExt;

use sandbox::app::create_app;

mod common;
use common::utils::{to_json, RequestBuilderExt};

async fn assert_error_response(response: Response<Body>) {
    let json = to_json(response.into_body()).await;
    let data = json.as_object().unwrap();
    assert!(data["message"].is_string());
}

#[sqlx::test]
async fn create_todo_success(pool: PgPool) {
    let app = create_app(pool);

    let payload = json!({ "title": "Some Task" });
    let request = Request::post("/todos").json(payload.clone());
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::CREATED);

    let json = to_json(response.into_body()).await;
    let todo = json.as_object().unwrap();

    assert_eq!(todo["title"], payload["title"]);
    assert_eq!(todo["completed"], false);
}

#[sqlx::test]
async fn create_todo_rejects_invalid_payload(pool: PgPool) {
    let app = create_app(pool);

    let payload = json!({ "description": "Some Task" });
    let request = Request::post("/todos").json(payload.clone());
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_error_response(response).await;
}

#[sqlx::test(fixtures("todos"))]
async fn list_todos_success(pool: PgPool) {
    let app = create_app(pool);

    let request = Request::get("/todos").empty_body();
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json = to_json(response.into_body()).await;
    let todos = json.as_array().unwrap();

    assert_eq!(todos.len(), 2);
    assert_eq!(todos[0]["id"], 1);
    assert_eq!(todos[0]["title"], "Buy groceries");
    assert_eq!(todos[0]["completed"], false);
    assert_eq!(todos[1]["id"], 2);
    assert_eq!(todos[1]["title"], "Do homework");
    assert_eq!(todos[1]["completed"], true);
}

#[sqlx::test(fixtures("todos"))]
async fn get_todo_success(pool: PgPool) {
    let app = create_app(pool);

    let request = Request::get("/todos/1").empty_body();
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json = to_json(response.into_body()).await;
    let todo = json.as_object().unwrap();

    assert_eq!(todo["id"], 1);
    assert_eq!(todo["title"], "Buy groceries");
    assert_eq!(todo["completed"], false);
}

#[sqlx::test(fixtures("todos"))]
async fn get_todo_rejects_unknwon_id(pool: PgPool) {
    let app = create_app(pool);

    let request = Request::get("/todos/42").empty_body();
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert_error_response(response).await;
}

#[sqlx::test(fixtures("todos"))]
async fn update_todo_success(pool: PgPool) {
    let app = create_app(pool);

    let payload = json!({ "title": "Some Task", "completed": true });
    let request = Request::put("/todos/1").json(payload.clone());
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let json = to_json(response.into_body()).await;
    let todo = json.as_object().unwrap();

    assert_eq!(todo["id"], 1);
    assert_eq!(todo["title"], payload["title"]);
    assert_eq!(todo["completed"], payload["completed"]);
}

#[sqlx::test(fixtures("todos"))]
async fn update_todo_rejects_invalid_payload(pool: PgPool) {
    let app = create_app(pool);

    let payload = json!({ "completed": "true" });
    let request = Request::put("/todos/1").json(payload.clone());
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);
    assert_error_response(response).await;
}

#[sqlx::test(fixtures("todos"))]
async fn update_todo_rejects_unknown_id(pool: PgPool) {
    let app = create_app(pool);

    let payload = json!({ "title": "Some Task", "completed": true });
    let request = Request::put("/todos/42").json(payload.clone());
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert_error_response(response).await;
}

#[sqlx::test(fixtures("todos"))]
async fn delete_todo_success(pool: PgPool) {
    let mut app = create_app(pool);

    let request = Request::delete("/todos/1").empty_body();
    let response = app.borrow_mut().oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NO_CONTENT);

    let request = Request::get("/todos/1").empty_body();
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[sqlx::test(fixtures("todos"))]
async fn delete_todo_rejects_unknown_id(pool: PgPool) {
    let app = create_app(pool);

    let request = Request::delete("/todos/42").empty_body();
    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
    assert_error_response(response).await;
}
