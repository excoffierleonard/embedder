//! Error handling for the crate.
//!
//! This module defines common error types and their integration with the Actix Web framework.

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

/// The response body for an error response
#[derive(Serialize)]
pub struct ErrorResponse {
    /// The error message
    pub message: String,
}

/// An error that can be returned from an API endpoint
#[derive(Debug)]
pub enum ApiError {
    /// A bad request error
    BadRequest(String),
    /// An internal server error
    InternalError(String),
}

impl std::fmt::Display for ApiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ApiError::BadRequest(msg) => write!(f, "Bad Request: {}", msg),
            ApiError::InternalError(msg) => write!(f, "Internal Error: {}", msg),
        }
    }
}

impl std::error::Error for ApiError {}

impl ResponseError for ApiError {
    fn error_response(&self) -> HttpResponse {
        let error_response = ErrorResponse {
            message: self.to_string(),
        };

        match self {
            ApiError::BadRequest(_) => HttpResponse::BadRequest().json(error_response),
            ApiError::InternalError(_) => HttpResponse::InternalServerError().json(error_response),
        }
    }

    fn status_code(&self) -> StatusCode {
        match self {
            ApiError::BadRequest(_) => StatusCode::BAD_REQUEST,
            ApiError::InternalError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<embedder_core::EmbedderError> for ApiError {
    fn from(err: embedder_core::EmbedderError) -> Self {
        ApiError::InternalError(err.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_display() {
        // Test each error variant's Display implementation
        let bad_request = ApiError::BadRequest("invalid request".to_string());
        let internal_error = ApiError::InternalError("server error".to_string());

        assert_eq!(bad_request.to_string(), "Bad Request: invalid request");
        assert_eq!(internal_error.to_string(), "Internal Error: server error");
    }

    #[test]
    fn status_codes() {
        // Test each error variant's status code
        assert_eq!(
            ApiError::BadRequest("test".to_string()).status_code(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            ApiError::InternalError("test".to_string()).status_code(),
            StatusCode::INTERNAL_SERVER_ERROR
        );
    }
}
