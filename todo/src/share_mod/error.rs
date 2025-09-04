use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

#[derive(Debug)]
pub enum AppError {
    DbError(sqlx::Error),
}


impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            AppError::DbError(err) => {
                eprintln!("Database error: {}", err);
                (StatusCode::INTERNAL_SERVER_ERROR, "Database query failed")
            }
        };
        
        (status, error_message).into_response()
    }
}
impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::DbError(err)
    }
}