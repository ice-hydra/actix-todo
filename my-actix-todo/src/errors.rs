use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Debug)]
#[allow(unused)]
pub enum AppErrorType {
    DbError,
    NotFoundError,
}

#[derive(Debug)]
pub struct AppError {
    pub message: Option<String>,
    pub cause: Option<String>,
    pub error_type: AppErrorType,
}

impl AppError {
    pub fn message(&self) -> String {
        match &*self {
            AppError {
                message: Some(msg),
                cause: _,
                error_type: AppErrorType::DbError,
            } => msg.clone(),
            AppError {
                message: None,
                cause: _,
                error_type: AppErrorType::NotFoundError,
            } => "The required item was not found".to_string(),
            _ => "An unexpected error has occurred".to_string(),
        }
    }
    pub fn db_error(err: impl ToString) -> AppError {
        AppError {
            message: None,
            cause: Some(err.to_string()),
            error_type: AppErrorType::DbError,
        }
    }
}

impl std::fmt::Display for AppError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self.error_type {
            AppErrorType::DbError => StatusCode::INTERNAL_SERVER_ERROR,
            AppErrorType::NotFoundError => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(AppErrorResponse {
            error: self.message(),
        })
    }
}

#[derive(Serialize)]
pub struct AppErrorResponse {
    pub error: String,
}
