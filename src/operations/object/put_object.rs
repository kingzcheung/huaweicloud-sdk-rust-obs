//! PutObject operation - upload an object to OBS.

use std::collections::HashMap;

use reqwest::{
    header::{HeaderMap, HeaderValue},
    Method,
};

use crate::client::Client;
use crate::error::{ObsError, Result};

/// Fluent builder for the PutObject operation.
#[derive(Debug, Clone)]
pub struct PutObjectFluentBuilder {
    client: Client,
    inner: PutObjectInput,
}

impl PutObjectFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: PutObjectInput::default(),
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

    /// Set the object body.
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.inner.body = Some(body);
        self
    }

    /// Set the content type.
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.inner.content_type = Some(content_type.into());
        self
    }

    /// Set the content encoding.
    pub fn content_encoding(mut self, content_encoding: impl Into<String>) -> Self {
        self.inner.content_encoding = Some(content_encoding.into());
        self
    }

    /// Set the content disposition.
    pub fn content_disposition(mut self, content_disposition: impl Into<String>) -> Self {
        self.inner.content_disposition = Some(content_disposition.into());
        self
    }

    /// Set the cache control.
    pub fn cache_control(mut self, cache_control: impl Into<String>) -> Self {
        self.inner.cache_control = Some(cache_control.into());
        self
    }

    /// Set the storage class.
    pub fn storage_class(mut self, storage_class: impl Into<String>) -> Self {
        self.inner.storage_class = Some(storage_class.into());
        self
    }

    /// Set custom metadata.
    pub fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.inner.metadata = Some(metadata);
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<PutObjectOutput> {
        let bucket = &self.inner.bucket;
        let key = &self.inner.key;

        if bucket.is_empty() {
            return Err(ObsError::InvalidInput("bucket name is required".to_string()));
        }
        if key.is_empty() {
            return Err(ObsError::InvalidInput("object key is required".to_string()));
        }

        let body = self.inner.body.clone().unwrap_or_default();
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

        if let Some(ref content_encoding) = self.inner.content_encoding {
            headers.insert(
                "Content-Encoding",
                HeaderValue::from_str(content_encoding).unwrap(),
            );
        }

        if let Some(ref content_disposition) = self.inner.content_disposition {
            headers.insert(
                "Content-Disposition",
                HeaderValue::from_str(content_disposition).unwrap(),
            );
        }

        if let Some(ref cache_control) = self.inner.cache_control {
            headers.insert(
                "Cache-Control",
                HeaderValue::from_str(cache_control).unwrap(),
            );
        }

        if let Some(ref storage_class) = self.inner.storage_class {
            headers.insert(
                "x-obs-storage-class",
                HeaderValue::from_str(storage_class).unwrap(),
            );
        }

        if let Some(ref metadata) = self.inner.metadata {
            for (k, v) in metadata {
                let header_name: reqwest::header::HeaderName =
                    format!("x-obs-meta-{}", k).parse().unwrap();
                headers.insert(header_name, HeaderValue::from_str(v).unwrap());
            }
        }

        let resp = self
            .client
            .do_request(Method::PUT, Some(bucket), Some(key), Some(headers), None, Some(body))
            .await?;

        let status = resp.status();
        let response_headers = resp.headers().clone();

        if !status.is_success() {
            let text = resp.text().await?;
            return Err(ObsError::service_error(status, &text));
        }

        let etag = response_headers
            .get("ETag")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.trim_matches('"').to_string());

        let request_id = response_headers
            .get("x-obs-request-id")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        Ok(PutObjectOutput { etag, request_id })
    }
}

/// Input for the PutObject operation.
#[derive(Debug, Clone, Default)]
pub struct PutObjectInput {
    bucket: String,
    key: String,
    body: Option<Vec<u8>>,
    content_type: Option<String>,
    content_encoding: Option<String>,
    content_disposition: Option<String>,
    cache_control: Option<String>,
    storage_class: Option<String>,
    metadata: Option<HashMap<String, String>>,
}

/// Output for the PutObject operation.
#[derive(Debug, Clone)]
pub struct PutObjectOutput {
    etag: Option<String>,
    request_id: Option<String>,
}

impl PutObjectOutput {
    /// Get the ETag.
    pub fn etag(&self) -> Option<&str> {
        self.etag.as_deref()
    }

    /// Get the request ID.
    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }
}