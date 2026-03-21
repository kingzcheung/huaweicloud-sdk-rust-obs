//! Copy Part operation.

use std::collections::HashMap;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::{ObsError, Result};

/// Fluent builder for the CopyPart operation.
///
/// This operation copies a part from an existing object to a multipart upload.
#[derive(Debug, Clone)]
pub struct CopyPartFluentBuilder {
    client: Client,
    inner: CopyPartInput,
}

impl CopyPartFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: CopyPartInput::default(),
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

    /// Set the upload ID.
    pub fn upload_id(mut self, upload_id: impl Into<String>) -> Self {
        self.inner.upload_id = upload_id.into();
        self
    }

    /// Set the part number (1-10000).
    pub fn part_number(mut self, part_number: i32) -> Self {
        self.inner.part_number = part_number;
        self
    }

    /// Set the copy source.
    ///
    /// Format: /SourceBucketName/SourceObjectName
    pub fn copy_source(mut self, copy_source: impl Into<String>) -> Self {
        self.inner.copy_source = Some(copy_source.into());
        self
    }

    /// Set the copy source range.
    ///
    /// Format: bytes=start-end
    pub fn copy_source_range(mut self, range: impl Into<String>) -> Self {
        self.inner.copy_source_range = Some(range.into());
        self
    }

    /// Set the customer algorithm for SSE-C (destination).
    pub fn ssec_customer_algorithm(mut self, algorithm: impl Into<String>) -> Self {
        self.inner.ssec_customer_algorithm = Some(algorithm.into());
        self
    }

    /// Set the customer key for SSE-C (destination).
    pub fn ssec_customer_key(mut self, key: impl Into<String>) -> Self {
        self.inner.ssec_customer_key = Some(key.into());
        self
    }

    /// Set the customer key MD5 for SSE-C (destination).
    pub fn ssec_customer_key_md5(mut self, md5: impl Into<String>) -> Self {
        self.inner.ssec_customer_key_md5 = Some(md5.into());
        self
    }

    /// Set the customer algorithm for SSE-C (source).
    pub fn copy_source_ssec_customer_algorithm(mut self, algorithm: impl Into<String>) -> Self {
        self.inner.copy_source_ssec_customer_algorithm = Some(algorithm.into());
        self
    }

    /// Set the customer key for SSE-C (source).
    pub fn copy_source_ssec_customer_key(mut self, key: impl Into<String>) -> Self {
        self.inner.copy_source_ssec_customer_key = Some(key.into());
        self
    }

    /// Set the customer key MD5 for SSE-C (source).
    pub fn copy_source_ssec_customer_key_md5(mut self, md5: impl Into<String>) -> Self {
        self.inner.copy_source_ssec_customer_key_md5 = Some(md5.into());
        self
    }

    /// Set the copy source if-match condition.
    ///
    /// Only copy if the source object's ETag matches this value.
    pub fn copy_source_if_match(mut self, etag: impl Into<String>) -> Self {
        self.inner.copy_source_if_match = Some(etag.into());
        self
    }

    /// Set the copy source if-none-match condition.
    ///
    /// Only copy if the source object's ETag does not match this value.
    pub fn copy_source_if_none_match(mut self, etag: impl Into<String>) -> Self {
        self.inner.copy_source_if_none_match = Some(etag.into());
        self
    }

    /// Set the copy source if-unmodified-since condition.
    ///
    /// Only copy if the source object has not been modified since this time.
    pub fn copy_source_if_unmodified_since(mut self, date: impl Into<String>) -> Self {
        self.inner.copy_source_if_unmodified_since = Some(date.into());
        self
    }

    /// Set the copy source if-modified-since condition.
    ///
    /// Only copy if the source object has been modified since this time.
    pub fn copy_source_if_modified_since(mut self, date: impl Into<String>) -> Self {
        self.inner.copy_source_if_modified_since = Some(date.into());
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<CopyPartOutput> {
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
        if self.inner.part_number < 1 || self.inner.part_number > 10000 {
            return Err(ObsError::InvalidInput("part number must be between 1 and 10000".to_string()));
        }
        if self.inner.copy_source.is_none() {
            return Err(ObsError::InvalidInput("copy source is required".to_string()));
        }

        let mut params = HashMap::new();
        params.insert("partNumber".to_string(), self.inner.part_number.to_string());
        params.insert("uploadId".to_string(), upload_id.clone());

        let mut headers = reqwest::header::HeaderMap::new();

        // Copy source
        if let Some(ref copy_source) = self.inner.copy_source {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(copy_source) {
                headers.insert("x-obs-copy-source", value);
            }
        }

        // Copy source range
        if let Some(ref range) = self.inner.copy_source_range {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(range) {
                headers.insert("x-obs-copy-source-range", value);
            }
        }

        // SSE-C headers for destination
        if let Some(ref algorithm) = self.inner.ssec_customer_algorithm {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(algorithm) {
                headers.insert("x-obs-server-side-encryption-customer-algorithm", value);
            }
        }

        if let Some(ref key) = self.inner.ssec_customer_key {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(key) {
                headers.insert("x-obs-server-side-encryption-customer-key", value);
            }
        }

        if let Some(ref md5) = self.inner.ssec_customer_key_md5 {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(md5) {
                headers.insert("x-obs-server-side-encryption-customer-key-MD5", value);
            }
        }

        // SSE-C headers for source
        if let Some(ref algorithm) = self.inner.copy_source_ssec_customer_algorithm {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(algorithm) {
                headers.insert("x-obs-copy-source-server-side-encryption-customer-algorithm", value);
            }
        }

        if let Some(ref key) = self.inner.copy_source_ssec_customer_key {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(key) {
                headers.insert("x-obs-copy-source-server-side-encryption-customer-key", value);
            }
        }

        if let Some(ref md5) = self.inner.copy_source_ssec_customer_key_md5 {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(md5) {
                headers.insert("x-obs-copy-source-server-side-encryption-customer-key-MD5", value);
            }
        }

        // Conditional copy headers
        if let Some(ref etag) = self.inner.copy_source_if_match {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(etag) {
                headers.insert("x-obs-copy-source-if-match", value);
            }
        }

        if let Some(ref etag) = self.inner.copy_source_if_none_match {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(etag) {
                headers.insert("x-obs-copy-source-if-none-match", value);
            }
        }

        if let Some(ref date) = self.inner.copy_source_if_unmodified_since {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(date) {
                headers.insert("x-obs-copy-source-if-unmodified-since", value);
            }
        }

        if let Some(ref date) = self.inner.copy_source_if_modified_since {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(date) {
                headers.insert("x-obs-copy-source-if-modified-since", value);
            }
        }

        let resp = self
            .client
            .do_request(Method::PUT, Some(bucket), Some(key), Some(headers), Some(params), None)
            .await?;

        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            return Err(ObsError::service_error(status, &text));
        }

        let result: CopyPartResultXml =
            crate::xml_utils::from_xml(&text)?;

        Ok(CopyPartOutput::from(result, self.inner.part_number))
    }
}

/// Input for the CopyPart operation.
#[derive(Debug, Clone, Default)]
pub struct CopyPartInput {
    bucket: String,
    key: String,
    upload_id: String,
    part_number: i32,
    copy_source: Option<String>,
    copy_source_range: Option<String>,
    ssec_customer_algorithm: Option<String>,
    ssec_customer_key: Option<String>,
    ssec_customer_key_md5: Option<String>,
    copy_source_ssec_customer_algorithm: Option<String>,
    copy_source_ssec_customer_key: Option<String>,
    copy_source_ssec_customer_key_md5: Option<String>,
    copy_source_if_match: Option<String>,
    copy_source_if_none_match: Option<String>,
    copy_source_if_unmodified_since: Option<String>,
    copy_source_if_modified_since: Option<String>,
}

/// Output for the CopyPart operation.
#[derive(Debug, Clone)]
pub struct CopyPartOutput {
    etag: String,
    last_modified: String,
    part_number: i32,
    crc64: Option<String>,
}

impl CopyPartOutput {
    /// Get the ETag of the copied part.
    pub fn etag(&self) -> &str {
        &self.etag
    }

    /// Get the last modified time.
    pub fn last_modified(&self) -> &str {
        &self.last_modified
    }

    /// Get the part number.
    pub fn part_number(&self) -> i32 {
        self.part_number
    }

    /// Get the CRC64 checksum.
    pub fn crc64(&self) -> Option<&str> {
        self.crc64.as_deref()
    }
}

impl CopyPartOutput {
    fn from(value: CopyPartResultXml, part_number: i32) -> Self {
        Self {
            etag: value.etag.trim_matches('"').to_string(),
            last_modified: value.last_modified,
            part_number,
            crc64: value.crc64,
        }
    }
}

// Internal XML response type for CopyPart
#[derive(Debug, Serialize, Deserialize)]
struct CopyPartResultXml {
    #[serde(rename = "LastModified")]
    last_modified: String,
    #[serde(rename = "ETag")]
    etag: String,
    #[serde(rename = "CRC64")]
    crc64: Option<String>,
}