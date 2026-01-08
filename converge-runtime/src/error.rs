// Copyright 2024-2025 Aprio One AB, Sweden
// Author: Kenneth Pernyer, kenneth@aprio.one
// SPDX-License-Identifier: MIT
// See LICENSE file in the project root for full license information.

//! Error types for Converge Runtime.

use converge_core::ConvergeError;
use serde::Serialize;
use thiserror::Error;
use utoipa::ToSchema;

/// Runtime-level errors.
#[derive(Debug, Error, ToSchema)]
#[schema(as = RuntimeErrorResponse)]
pub enum RuntimeError {
    /// Converge engine error.
    #[error("converge error: {0}")]
    Converge(#[from] ConvergeError),

    /// Serialization error.
    #[error("serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// HTTP server error.
    #[error("HTTP error: {0}")]
    Http(#[from] axum::Error),

    /// I/O error.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Configuration error.
    #[error("configuration error: {0}")]
    Config(String),
}

/// Error response for API.
#[derive(Debug, Serialize, ToSchema)]
pub struct RuntimeErrorResponse {
    /// Error message.
    pub error: String,
    /// HTTP status code.
    pub status: u16,
}

/// Result type for runtime operations.
pub type RuntimeResult<T> = Result<T, RuntimeError>;

impl axum::response::IntoResponse for RuntimeError {
    fn into_response(self) -> axum::response::Response {
        let (status, message) = match self {
            RuntimeError::Converge(e) => {
                let status = match e {
                    ConvergeError::BudgetExhausted { .. } => {
                        axum::http::StatusCode::PAYLOAD_TOO_LARGE
                    }
                    ConvergeError::InvariantViolation { .. } => {
                        axum::http::StatusCode::UNPROCESSABLE_ENTITY
                    }
                    ConvergeError::AgentFailed { .. } => {
                        axum::http::StatusCode::INTERNAL_SERVER_ERROR
                    }
                    ConvergeError::Conflict { .. } => axum::http::StatusCode::CONFLICT,
                };
                (status, format!("Converge error: {}", e))
            }
            RuntimeError::Serialization(e) => (
                axum::http::StatusCode::BAD_REQUEST,
                format!("Invalid JSON: {}", e),
            ),
            RuntimeError::Http(e) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("HTTP error: {}", e),
            ),
            RuntimeError::Io(e) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("I/O error: {}", e),
            ),
            RuntimeError::Config(msg) => (
                axum::http::StatusCode::INTERNAL_SERVER_ERROR,
                format!("Configuration error: {}", msg),
            ),
        };

        let body = RuntimeErrorResponse {
            error: message,
            status: status.as_u16(),
        };

        (status, axum::Json(body)).into_response()
    }
}
