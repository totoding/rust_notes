use axum::{
    Json, Router,
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use sqlx::{Pool, Postgres};

use super::model::{CreateTodo, QueryParams};
use super::service::TodoService;
use crate::share_mod::AppError;
use crate::share_mod::response::ApiResponse;
pub fn todo_routes() -> Router<Pool<Postgres>> {
    Router::new()
        .route("/", get(get_handler))
        .route("/", post(post_handler))
        .route("/{id}", put(put_handler))
        .route("/{id}", delete(delete_handler))
}

async fn get_handler(
    State(pool): State<Pool<Postgres>>,
    Query(params): Query<QueryParams>,
) -> Result<impl IntoResponse, AppError> {
    let todos = TodoService::get_todos(&pool, params).await?;
    Ok(ApiResponse::success(todos))
}

async fn post_handler(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<CreateTodo>,
) -> Result<impl IntoResponse, AppError> {
    let result = TodoService::create_todo(&pool, payload).await?;
    Ok(ApiResponse::success(result))
}

async fn delete_handler(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    TodoService::delete_todo(&pool, id).await?;
    Ok(ApiResponse::success(()))
}

async fn put_handler(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateTodo>,
) -> Result<impl IntoResponse, AppError> {
    let result = TodoService::update_todo(&pool, id, payload).await?;
    Ok(ApiResponse::success(result))
}
