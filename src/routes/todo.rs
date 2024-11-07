use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::routing::get;
use axum::Router;
use serde::Deserialize;

use crate::app::AppState;
use crate::error::ApiError;
use crate::extractor::Json;
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
) -> Result<(StatusCode, Json<Todo>), ApiError> {
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
    .map_err(|e| {
        tracing::error!("Failed to create todo: {:?}", e);
        ApiError::InternalError(format!("{}", e))
    })?;

    let todo = Todo::new(row.id, row.title.clone(), row.completed);
    Ok((StatusCode::CREATED, Json(todo)))
}

async fn list_todos(
    State(state): State<AppState>,
) -> Result<(StatusCode, Json<Vec<Todo>>), ApiError> {
    let rows = sqlx::query!("SELECT * FROM todos")
        .fetch_all(&state.db_pool)
        .await
        .map_err(|e| {
            tracing::error!("Failed to list todos: {:?}", e);
            ApiError::InternalError(format!("{}", e))
        })?;

    let todos = rows
        .iter()
        .map(|row| Todo::new(row.id, row.title.clone(), row.completed))
        .collect();
    Ok((StatusCode::OK, Json(todos)))
}

async fn get_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<(StatusCode, Json<Todo>), ApiError> {
    let row = sqlx::query!("SELECT * FROM todos WHERE id = $1", id)
        .fetch_one(&state.db_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                ApiError::NotFound(format!("Todo with id \"{}\" not found", id))
            }
            _ => {
                tracing::error!("Failed to get todo: {:?}", e);
                ApiError::InternalError(format!("{}", e))
            }
        })?;
    let todo = Todo::new(row.id, row.title.clone(), row.completed);
    Ok((StatusCode::OK, Json(todo)))
}

async fn update_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Json(body): Json<UpdateTodoBody>,
) -> Result<(StatusCode, Json<Todo>), ApiError> {
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
    .map_err(|e| match e {
        sqlx::Error::RowNotFound => {
            ApiError::NotFound(format!("Todo with id \"{}\" not found", id))
        }
        _ => {
            tracing::error!("Failed to update todo: {:?}", e);
            ApiError::InternalError(format!("{}", e))
        }
    })?;

    let todo = Todo::new(row.id, row.title.clone(), row.completed);
    Ok((StatusCode::OK, Json(todo)))
}

async fn delete_todo(
    State(state): State<AppState>,
    Path(id): Path<i32>,
) -> Result<StatusCode, ApiError> {
    sqlx::query!("DELETE FROM todos WHERE id = $1", id)
        .execute(&state.db_pool)
        .await
        .map_err(|e| match e {
            sqlx::Error::RowNotFound => {
                ApiError::NotFound(format!("Todo with id \"{}\" not found", id))
            }
            _ => {
                tracing::error!("Failed to delete todo: {:?}", e);
                ApiError::InternalError(format!("{}", e))
            }
        })?;

    Ok(StatusCode::NO_CONTENT)
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(list_todos).post(create_todo))
        .route("/:id", get(get_todo).put(update_todo).delete(delete_todo))
}
