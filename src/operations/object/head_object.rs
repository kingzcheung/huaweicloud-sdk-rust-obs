//! HeadObject operation - get object metadata from OBS.

use std::collections::HashMap;

use reqwest::Method;

use crate::client::Client;
use crate::error::{ObsError, Result};

/// Fluent builder for the HeadObject operation.
#[derive(Debug, Clone)]
pub struct HeadObjectFluentBuilder {
    client: Client,
    inner: HeadObjectInput,
}

impl HeadObjectFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: HeadObjectInput::default(),
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

    /// Set the version ID.
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.inner.version_id = Some(version_id.into());
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<HeadObjectOutput> {
        let bucket = &self.inner.bucket;
        let key = &self.inner.key;

        if bucket.is_empty() {
            return Err(ObsError::InvalidInput(
                "bucket name is required".to_string(),
            ));
        }
        if key.is_empty() {
            return Err(ObsError::InvalidInput("object key is required".to_string()));
        }

        let mut params = HashMap::new();
        if let Some(ref version_id) = self.inner.version_id {
            params.insert("versionId".to_string(), version_id.clone());
        }

        let resp = self
            .client
            .do_request(
                Method::HEAD,
                Some(bucket),
                Some(key),
                None,
                Some(params),
                None,
            )
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(ObsError::service_error(status, &text));
        }

        let headers = resp.headers();

        let content_type = headers
            .get("Content-Type")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let content_length = headers
            .get("Content-Length")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok());

        let etag = headers
            .get("ETag")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.trim_matches('"').to_string());

        let last_modified = headers
            .get("Last-Modified")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let storage_class = headers
            .get("x-obs-storage-class")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        Ok(HeadObjectOutput {
            content_type,
            content_length,
            etag,
            last_modified,
            storage_class,
        })
    }
}

/// Input for the HeadObject operation.
#[derive(Debug, Clone, Default)]
pub struct HeadObjectInput {
    bucket: String,
    key: String,
    version_id: Option<String>,
}

/// Output for the HeadObject operation.
#[derive(Debug, Clone)]
pub struct HeadObjectOutput {
    content_type: Option<String>,
    content_length: Option<u64>,
    etag: Option<String>,
    last_modified: Option<String>,
    storage_class: Option<String>,
}

impl HeadObjectOutput {
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

    /// Get the storage class.
    pub fn storage_class(&self) -> Option<&str> {
        self.storage_class.as_deref()
    }
}
