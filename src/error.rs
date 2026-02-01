use crate::db;
use axum::response::{IntoResponse, Response};
use http::StatusCode;

pub enum AppError {
    Db(rusqlite::Error),
}

impl From<rusqlite::Error> for AppError {
    fn from(value: rusqlite::Error) -> Self {
        AppError::Db(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}
