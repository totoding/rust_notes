use axum::{
    Json, Router,
    extract::{Path, Query, State},
    response::IntoResponse,
    routing::{delete, get, post, put},
};
use sqlx::{Pool, Postgres, QueryBuilder};

use super::model::{Todo, CreateTodo, QueryParams};
use crate::share_mod::AppError;

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
    let mut query_builder: QueryBuilder<Postgres> = QueryBuilder::new(
        "SELECT id, title, done, created_at, deleted_at FROM todos WHERE deleted_at IS NULL",
    );

    if let Some(title) = &params.title {
        query_builder.push(" AND title ILIKE ");
        query_builder.push_bind(format!("%{}%", title));
    }

    if let Some(done) = params.done {
        query_builder.push(" AND done = ");
        query_builder.push_bind(done);
    }

    query_builder.push(" ORDER BY id ASC LIMIT ");
    query_builder.push_bind(params.page_size);
    query_builder.push(" OFFSET ");
    query_builder.push_bind((params.page - 1) * params.page_size);

    let todos = query_builder
        .build_query_as::<Todo>()
        .fetch_all(&pool)
        .await?;

    Ok(Json(todos))
}

async fn post_handler(
    State(pool): State<Pool<Postgres>>,
    Json(payload): Json<CreateTodo>,
) -> Result<impl IntoResponse, AppError> {
    let result = sqlx::query_as!(
        Todo,
        r#"
        INSERT INTO todos (title, done)
        VALUES ($1, $2)
        RETURNING id, title, done, created_at, deleted_at;
        "#,
        payload.title,
        payload.done
    )
    .fetch_one(&pool)
    .await?;
    Ok(Json(result))
}

async fn delete_handler(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, AppError> {
    let now = chrono::Utc::now();
    sqlx::query!(
        r#"
            UPDATE todos
            SET deleted_at = $1
            WHERE id = $2;
            "#,
        now,
        id
    )
    .execute(&pool)
    .await?;

    Ok(Json(1))
}

async fn put_handler(
    State(pool): State<Pool<Postgres>>,
    Path(id): Path<i32>,
    Json(payload): Json<CreateTodo>
) -> Result<impl IntoResponse, AppError> {
    let result = sqlx::query_as!(
        Todo,
        r#"
            UPDATE todos
            SET title = $1, done = $2
            WHERE id = $3
            RETURNING id, title, done, created_at, deleted_at;
        "#,
        payload.title,
        payload.done,
        id
    )
    .fetch_one(&pool)
    .await?;
    Ok(Json(result))
}