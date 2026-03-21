//! DeleteObject operation - delete a single object from OBS.

use std::collections::HashMap;

use reqwest::Method;

use crate::client::Client;
use crate::error::{ObsError, Result};

/// Fluent builder for the DeleteObject operation.
#[derive(Debug, Clone)]
pub struct DeleteObjectFluentBuilder {
    client: Client,
    inner: DeleteObjectInput,
}

impl DeleteObjectFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: DeleteObjectInput::default(),
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
    pub async fn send(&self) -> Result<DeleteObjectOutput> {
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
                Method::DELETE,
                Some(bucket),
                Some(key),
                None,
                Some(params),
                None,
            )
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await?;
            return Err(ObsError::service_error(status, &text));
        }

        Ok(DeleteObjectOutput {})
    }
}

/// Input for the DeleteObject operation.
#[derive(Debug, Clone, Default)]
pub struct DeleteObjectInput {
    bucket: String,
    key: String,
    version_id: Option<String>,
}

/// Output for the DeleteObject operation.
#[derive(Debug, Clone)]
pub struct DeleteObjectOutput {}
