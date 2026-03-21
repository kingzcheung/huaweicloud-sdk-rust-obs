//! Abort Multipart Upload operation.

use std::collections::HashMap;

use reqwest::Method;

use crate::client::Client;
use crate::error::{ObsError, Result};

/// Fluent builder for the AbortMultipartUpload operation.
///
/// This operation aborts a multipart upload.
#[derive(Debug, Clone)]
pub struct AbortMultipartUploadFluentBuilder {
    client: Client,
    inner: AbortMultipartUploadInput,
}

impl AbortMultipartUploadFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: AbortMultipartUploadInput::default(),
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

    /// Set the upload ID.
    pub fn upload_id(mut self, upload_id: impl Into<String>) -> Self {
        self.inner.upload_id = upload_id.into();
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<AbortMultipartUploadOutput> {
        let bucket = &self.inner.bucket;
        let key = &self.inner.key;
        let upload_id = &self.inner.upload_id;
        
        if bucket.is_empty() {
            return Err(ObsError::InvalidInput("bucket name is required".to_string()));
        }
        if key.is_empty() {
            return Err(ObsError::InvalidInput("object key is required".to_string()));
        }
        if upload_id.is_empty() {
            return Err(ObsError::InvalidInput("upload ID is required".to_string()));
        }

        let mut params = HashMap::new();
        params.insert("uploadId".to_string(), upload_id.clone());

        let resp = self
            .client
            .do_request(Method::DELETE, Some(bucket), Some(key), None, Some(params), None)
            .await?;

        let status = resp.status();
        
        // AbortMultipartUpload returns 204 No Content on success
        if !status.is_success() {
            let text = resp.text().await?;
            return Err(ObsError::service_error(status, &text));
        }

        Ok(AbortMultipartUploadOutput {})
    }
}

/// Input for the AbortMultipartUpload operation.
#[derive(Debug, Clone, Default)]
pub struct AbortMultipartUploadInput {
    bucket: String,
    key: String,
    upload_id: String,
}

/// Output for the AbortMultipartUpload operation.
#[derive(Debug, Clone)]
pub struct AbortMultipartUploadOutput {}