//! AppendObject operation - append data to an object in OBS.

use std::collections::HashMap;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Method,
};

use crate::client::Client;
use crate::error::{ObsError, Result};

/// Fluent builder for the AppendObject operation.
#[derive(Debug, Clone)]
pub struct AppendObjectFluentBuilder {
    client: Client,
    inner: AppendObjectInput,
}

impl AppendObjectFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: AppendObjectInput::default(),
        }
    }

    /// Set the bucket name.
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.inner.bucket = bucket.into();
        self
    }

    /// Set the object key.
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.inner.key = key.into();
        self
    }

    /// Set the position to append at.
    pub fn position(mut self, position: u64) -> Self {
        self.inner.position = position;
        self
    }

    /// Set the body to append.
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.inner.body = Some(body);
        self
    }

    /// Set the content type.
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.inner.content_type = Some(content_type.into());
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<AppendObjectOutput> {
        let bucket = &self.inner.bucket;
        let key = &self.inner.key;

        if bucket.is_empty() {
            return Err(ObsError::InvalidInput("bucket name is required".to_string()));
        }
        if key.is_empty() {
            return Err(ObsError::InvalidInput("object key is required".to_string()));
        }

        let body = self.inner.body.clone().unwrap_or_default();

        let mut params = HashMap::new();
        params.insert("append".to_string(), String::new());
        params.insert("position".to_string(), self.inner.position.to_string());

        let mut headers = HeaderMap::new();
        headers.insert(
            "Content-Length",
            HeaderValue::from_str(&body.len().to_string()).unwrap(),
        );

        if let Some(ref content_type) = self.inner.content_type {
            headers.insert(
                "Content-Type",
                HeaderValue::from_str(content_type).unwrap(),
            );
        }

        let resp = self
            .client
            .do_request(Method::POST, Some(bucket), Some(key), Some(headers), Some(params), Some(body))
            .await?;

        let status = resp.status();
        let response_headers = resp.headers().clone();

        if !status.is_success() {
            let text = resp.text().await?;
            return Err(ObsError::service_error(status, &text));
        }

        let next_position = response_headers
            .get("x-obs-next-append-position")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok());

        let etag = response_headers
            .get("ETag")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.trim_matches('"').to_string());

        Ok(AppendObjectOutput {
            next_position,
            etag,
        })
    }
}

/// Input for the AppendObject operation.
#[derive(Debug, Clone, Default)]
pub struct AppendObjectInput {
    bucket: String,
    key: String,
    position: u64,
    body: Option<Vec<u8>>,
    content_type: Option<String>,
}

/// Output for the AppendObject operation.
#[derive(Debug, Clone)]
pub struct AppendObjectOutput {
    next_position: Option<u64>,
    etag: Option<String>,
}

impl AppendObjectOutput {
    /// Get the next append position.
    pub fn next_position(&self) -> Option<u64> {
        self.next_position
    }

    /// Get the ETag.
    pub fn etag(&self) -> Option<&str> {
        self.etag.as_deref()
    }
}