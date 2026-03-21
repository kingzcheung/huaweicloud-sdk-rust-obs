//! GetObject operation - download an object from OBS.

use std::collections::HashMap;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Method,
};

use crate::client::Client;
use crate::error::{ObsError, Result};

/// Fluent builder for the GetObject operation.
#[derive(Debug, Clone)]
pub struct GetObjectFluentBuilder {
    client: Client,
    inner: GetObjectInput,
}

impl GetObjectFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: GetObjectInput::default(),
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

    /// Set the range.
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.inner.range = Some(range.into());
        self
    }

    /// Set the version ID.
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.inner.version_id = Some(version_id.into());
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<GetObjectOutput> {
        let bucket = &self.inner.bucket;
        let key = &self.inner.key;

        if bucket.is_empty() {
            return Err(ObsError::InvalidInput("bucket name is required".to_string()));
        }
        if key.is_empty() {
            return Err(ObsError::InvalidInput("object key is required".to_string()));
        }

        let mut headers = HeaderMap::new();
        if let Some(ref range) = self.inner.range {
            headers.insert("Range", HeaderValue::from_str(range).unwrap());
        }

        let mut params = HashMap::new();
        if let Some(ref version_id) = self.inner.version_id {
            params.insert("versionId".to_string(), version_id.clone());
        }

        let resp = self
            .client
            .do_request(Method::GET, Some(bucket), Some(key), Some(headers), Some(params), None)
            .await?;

        let status = resp.status();
        let response_headers = resp.headers().clone();

        if !status.is_success() {
            let text = resp.text().await?;
            return Err(ObsError::service_error(status, &text));
        }

        let body = resp.bytes().await?;

        let content_type = response_headers
            .get("Content-Type")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let content_length = response_headers
            .get("Content-Length")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok());

        let etag = response_headers
            .get("ETag")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.trim_matches('"').to_string());

        let last_modified = response_headers
            .get("Last-Modified")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        Ok(GetObjectOutput {
            body,
            content_type,
            content_length,
            etag,
            last_modified,
        })
    }
}

/// Input for the GetObject operation.
#[derive(Debug, Clone, Default)]
pub struct GetObjectInput {
    bucket: String,
    key: String,
    range: Option<String>,
    version_id: Option<String>,
}

/// Output for the GetObject operation.
#[derive(Debug)]
pub struct GetObjectOutput {
    body: bytes::Bytes,
    content_type: Option<String>,
    content_length: Option<u64>,
    etag: Option<String>,
    last_modified: Option<String>,
}

impl GetObjectOutput {
    /// Get the body.
    pub fn body(&self) -> &bytes::Bytes {
        &self.body
    }

    /// Get the body as bytes.
    pub fn body_bytes(&self) -> &[u8] {
        &self.body
    }

    /// Consume the output and get the body.
    pub fn into_body(self) -> bytes::Bytes {
        self.body
    }

    /// Get the content type.
    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }

    /// Get the content length.
    pub fn content_length(&self) -> Option<u64> {
        self.content_length
    }

    /// Get the ETag.
    pub fn etag(&self) -> Option<&str> {
        self.etag.as_deref()
    }

    /// Get the last modified time.
    pub fn last_modified(&self) -> Option<&str> {
        self.last_modified.as_deref()
    }
}