use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use serde::Deserialize;

use crate::app::AppState;
use crate::models::todo::Todo;

#[derive(Deserialize)]
struct CreateTodoBody {
    title: String,
}

#[derive(Deserialize)]
struct UpdateTodoBody {
    title: Option<String>,
    completed: Option<bool>,
}

async fn create_todo(
    State(state): State<AppState>,
    Json(body): Json<CreateTodoBody>,
) -> (StatusCode, Json<Todo>) {
    let row = sqlx::query!(
        r#"
            INSERT INTO todos (title)
            VALUES ($1)
            RETURNING id, title, completed
        "#,
        body.title
    )
    .fetch_one(&state.db_pool)
    .await
    .unwrap();

    let todo = Todo::new(row.id, row.title.clone(), row.completed);
    (StatusCode::CREATED, Json(todo))
}

async fn list_todos(State(state): State<AppState>) -> (StatusCode, Json<Vec<Todo>>) {
    let rows = sqlx::query!("SELECT * FROM todos")
        .fetch_all(&state.db_pool)
        .await
        .unwrap();

    let todos = rows
        .iter()
        .map(|row| Todo::new(row.id, row.title.clone(), row.completed))
        .collect();
    (StatusCode::OK, Json(todos))
}

async fn get_todo(State(state): State<AppState>, Path(id): Path<i32>) -> (StatusCode, Json<Todo>) {
    let row = sqlx::query!("SELECT * FROM todos WHERE id = $1", id)
        .fetch_one(&state.db_pool)
        .await
        .unwrap();
    let todo = Todo::new(row.id, row.title.clone(), row.completed);
    (StatusCode::OK, Json(todo))
}

async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateTodoBody>,
) -> (StatusCode, Json<Todo>) {
    let row = sqlx::query!(
        r#"
            UPDATE todos
            SET title = COALESCE($2, title),
                completed = COALESCE($3, completed)
            WHERE id = $1
            RETURNING id, title, completed
        "#,
        id,
        body.title,
        body.completed
    )
    .fetch_one(&state.db_pool)
    .await
    .unwrap();

    let todo = Todo::new(row.id, row.title.clone(), row.completed);
    (StatusCode::OK, Json(todo))
}

async fn delete_todo(State(state): State<AppState>, Path(id): Path<i32>) -> StatusCode {
    sqlx::query!("DELETE FROM todos WHERE id = $1", id)
        .execute(&state.db_pool)
        .await
        .unwrap();

    StatusCode::NO_CONTENT
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_todos).post(create_todo))
        .route("/:id", get(get_todo).put(update_todo).delete(delete_todo))
}
