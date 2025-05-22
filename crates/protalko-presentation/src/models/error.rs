use protalko_domain::value_objects::{error::ErrorModel, response_status::Status};

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

pub struct AppError {
    status_code: StatusCode,
    message: String,
}

pub fn status_code(status: &Status) -> StatusCode {
    match status {
        Status::Ok => StatusCode::OK,
        Status::Created => StatusCode::CREATED,
        Status::BadRequest => StatusCode::BAD_REQUEST,
        Status::Unauthorized => StatusCode::UNAUTHORIZED,
        Status::Forbidden => StatusCode::FORBIDDEN,
        Status::NotFound => StatusCode::NOT_FOUND,
        Status::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

impl From<ErrorModel> for AppError {
    fn from(value: ErrorModel) -> Self {
        Self {
            status_code: status_code(value.status()),
            message: value.message().to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    message: String,
}

impl From<AppError> for ErrorResponse {
    fn from(value: AppError) -> Self {
        Self {
            message: value.message.to_string(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        (self.status_code, Json(ErrorResponse::from(self))).into_response()
    }
}
