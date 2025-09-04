use axum::response::{IntoResponse, Response};

use super::response::ApiResponse;

#[derive(Debug)]
pub enum AppError {
    DbError(sqlx::Error),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (code, msg) = match self {
            AppError::DbError(err) => {
                eprintln!("Database error: {}", err);
                (500, "数据库操作失败")
            }
        };

        ApiResponse::error(code, msg).into_response()
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DbError(err)
    }
}
