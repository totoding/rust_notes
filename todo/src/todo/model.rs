use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(Debug, FromRow, Serialize)]
pub struct Todo {
    pub id: i32,
    pub title: String,
    pub done: bool,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub deleted_at: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QueryParams {
    pub page: i64,
    pub page_size: i64,
    pub title: Option<String>,
    pub done: Option<bool>,
}