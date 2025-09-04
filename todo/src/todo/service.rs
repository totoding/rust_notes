use sqlx::{Pool, Postgres, QueryBuilder};

use super::model::{Todo, CreateTodo, QueryParams};
use crate::share_mod::AppError;

pub struct TodoService;

impl TodoService {
    pub async fn get_todos(
        pool: &Pool<Postgres>,
        params: QueryParams,
    ) -> Result<Vec<Todo>, AppError> {
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
            .fetch_all(pool)
            .await?;

        Ok(todos)
    }

    pub async fn create_todo(
        pool: &Pool<Postgres>,
        payload: CreateTodo,
    ) -> Result<Todo, AppError> {
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
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn update_todo(
        pool: &Pool<Postgres>,
        id: i32,
        payload: CreateTodo,
    ) -> Result<Todo, AppError> {
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
        .fetch_one(pool)
        .await?;

        Ok(result)
    }

    pub async fn delete_todo(
        pool: &Pool<Postgres>,
        id: i32,
    ) -> Result<(), AppError> {
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
        .execute(pool)
        .await?;

        Ok(())
    }
}