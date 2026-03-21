//! Error types for the OBS SDK.
//!
//! This module provides comprehensive error handling for all OBS operations.

use reqwest::StatusCode;
use serde::Deserialize;
use thiserror::Error;

/// The main error type for OBS SDK operations.
#[derive(Error, Debug)]
pub enum ObsError {
    /// Error occurred while building the HTTP client.
    #[error("failed to build HTTP client: {0}")]
    ClientBuild(String),

    /// Error occurred during the HTTP request.
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),

    /// Invalid credentials provided.
    #[error("invalid credentials: {0}")]
    Credentials(String),

    /// Error response from the OBS service.
    #[error("OBS service error: {status} - {message}")]
    ServiceError {
        /// HTTP status code
        status: StatusCode,
        /// Error code from OBS
        code: Option<String>,
        /// Human-readable error message
        message: String,
        /// Request ID for debugging
        request_id: Option<String>,
        /// Host ID for debugging
        host_id: Option<String>,
    },

    /// Error occurred while parsing XML response.
    #[error("failed to parse XML response: {0}")]
    XmlParse(String),

    /// Error occurred while serializing request body.
    #[error("failed to serialize request: {0}")]
    Serialization(String),

    /// Invalid input parameter.
    #[error("invalid input: {0}")]
    InvalidInput(String),

    /// IO error occurred.
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    /// Unknown error occurred.
    #[error("unknown error: {0}")]
    Unknown(String),
}

/// Error response from OBS service.
#[derive(Debug, Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Message")]
    pub message: String,
    #[serde(rename = "RequestId")]
    pub request_id: Option<String>,
    #[serde(rename = "HostId")]
    pub host_id: Option<String>,
    #[serde(rename = "AccessKeyId", skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,
    #[serde(rename = "SignatureProvided", skip_serializing_if = "Option::is_none")]
    pub signature_provided: Option<String>,
    #[serde(rename = "StringToSign", skip_serializing_if = "Option::is_none")]
    pub string_to_sign: Option<String>,
}

impl ObsError {
    /// Create a new service error from a response.
    pub fn service_error(status: StatusCode, body: &str) -> Self {
        // Try to parse the error response
        match crate::xml_utils::from_xml::<ErrorResponse>(body) {
            Ok(err) => ObsError::ServiceError {
                status,
                code: Some(err.code),
                message: err.message,
                request_id: err.request_id,
                host_id: err.host_id,
            },
            Err(_) => ObsError::ServiceError {
                status,
                code: None,
                message: body.to_string(),
                request_id: None,
                host_id: None,
            },
        }
    }

    /// Check if this error is retryable.
    pub fn is_retryable(&self) -> bool {
        match self {
            ObsError::HttpError(e) => e.is_timeout() || e.is_connect(),
            ObsError::ServiceError { status, .. } => {
                matches!(
                    status,
                    &StatusCode::INTERNAL_SERVER_ERROR
                        | &StatusCode::BAD_GATEWAY
                        | &StatusCode::SERVICE_UNAVAILABLE
                        | &StatusCode::GATEWAY_TIMEOUT
                        | &StatusCode::TOO_MANY_REQUESTS
                )
            }
            _ => false,
        }
    }

    /// Get the HTTP status code if this is a service error.
    pub fn status_code(&self) -> Option<StatusCode> {
        match self {
            ObsError::ServiceError { status, .. } => Some(*status),
            ObsError::HttpError(e) => e.status(),
            _ => None,
        }
    }
}

/// Result type alias for OBS operations.
pub type Result<T> = std::result::Result<T, ObsError>;
