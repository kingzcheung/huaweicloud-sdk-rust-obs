//! CopyObject operation - copy an object within OBS.

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Method,
};
use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::{ObsError, Result};

/// Fluent builder for the CopyObject operation.
#[derive(Debug, Clone)]
pub struct CopyObjectFluentBuilder {
    client: Client,
    inner: CopyObjectInput,
}

impl CopyObjectFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: CopyObjectInput::default(),
        }
    }

    /// Set the destination bucket name.
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.inner.bucket = bucket.into();
        self
    }

    /// Set the destination object key.
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.inner.key = key.into();
        self
    }

    /// Set the copy source (format: "source-bucket/source-key").
    pub fn copy_source(mut self, copy_source: impl Into<String>) -> Self {
        self.inner.copy_source = copy_source.into();
        self
    }

    /// Set the content type.
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.inner.content_type = Some(content_type.into());
        self
    }

    /// Set the storage class.
    pub fn storage_class(mut self, storage_class: impl Into<String>) -> Self {
        self.inner.storage_class = Some(storage_class.into());
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<CopyObjectOutput> {
        let bucket = &self.inner.bucket;
        let key = &self.inner.key;

        if bucket.is_empty() {
            return Err(ObsError::InvalidInput("bucket name is required".to_string()));
        }
        if key.is_empty() {
            return Err(ObsError::InvalidInput("object key is required".to_string()));
        }
        if self.inner.copy_source.is_empty() {
            return Err(ObsError::InvalidInput("copy source is required".to_string()));
        }

        let mut headers = HeaderMap::new();

        // x-obs-copy-source 格式: /bucket/key
        // copy_source 格式为 "bucket/key"
        let copy_source = format!("/{}", self.inner.copy_source);

        headers.insert(
            "x-obs-copy-source",
            HeaderValue::from_str(&copy_source)
                .map_err(|e| ObsError::InvalidInput(format!("Invalid copy source: {}", e)))?,
        );

        if let Some(ref content_type) = self.inner.content_type {
            headers.insert(
                "Content-Type",
                HeaderValue::from_str(content_type).unwrap(),
            );
        }

        if let Some(ref storage_class) = self.inner.storage_class {
            headers.insert(
                "x-obs-storage-class",
                HeaderValue::from_str(storage_class).unwrap(),
            );
        }

        let resp = self
            .client
            .do_request(Method::PUT, Some(bucket), Some(key), Some(headers), None, None)
            .await?;

        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            return Err(ObsError::service_error(status, &text));
        }

        let result: CopyObjectResult =
            crate::xml_utils::from_xml(&text)?;

        Ok(CopyObjectOutput {
            etag: result.etag,
            last_modified: result.last_modified,
        })
    }
}

/// Input for the CopyObject operation.
#[derive(Debug, Clone, Default)]
pub struct CopyObjectInput {
    bucket: String,
    key: String,
    copy_source: String,
    content_type: Option<String>,
    storage_class: Option<String>,
}

/// Output for the CopyObject operation.
#[derive(Debug, Clone)]
pub struct CopyObjectOutput {
    etag: String,
    last_modified: String,
}

impl CopyObjectOutput {
    /// Get the ETag.
    pub fn etag(&self) -> &str {
        &self.etag
    }

    /// Get the last modified time.
    pub fn last_modified(&self) -> &str {
        &self.last_modified
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CopyObjectResult {
    #[serde(rename = "ETag")]
    etag: String,
    #[serde(rename = "LastModified")]
    last_modified: String,
}