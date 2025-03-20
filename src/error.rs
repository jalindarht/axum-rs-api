use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    DatabaseConnectionError(String),
    DatabaseError(String),
    TaskNotFound,
    BadRequest(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::DatabaseError(err) => (StatusCode::INTERNAL_SERVER_ERROR, "Database operation error: ".to_string() + &err),
            AppError::DatabaseConnectionError(err) => (StatusCode::INTERNAL_SERVER_ERROR, "Database connection error".to_string() + &err),
            AppError::TaskNotFound => (StatusCode::NOT_FOUND, "Task not found".to_string()),
            AppError::BadRequest(err) => (StatusCode::BAD_REQUEST, err),
        };
        (status, message).into_response()
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::DatabaseError(err) => write!(f, "Database error: {}", err),
            AppError::DatabaseConnectionError(err) => write!(f, "Database connection error: {}", err),
            AppError::TaskNotFound => write!(f, "Task not found"),
            AppError::BadRequest(err) => write!(f, "Bad request: {}", err),
        }
    }
}