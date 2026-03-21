//! Upload Part operation.

use std::collections::HashMap;

use reqwest::Method;

use crate::client::Client;
use crate::error::{ObsError, Result};

/// Fluent builder for the UploadPart operation.
///
/// This operation uploads a part in a multipart upload.
pub struct UploadPartFluentBuilder {
    client: Client,
    inner: UploadPartInput,
}

impl UploadPartFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: UploadPartInput::default(),
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

    /// Set the part number (1-10000).
    pub fn part_number(mut self, part_number: i32) -> Self {
        self.inner.part_number = part_number;
        self
    }

    /// Set the body content.
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.inner.body = Some(UploadPartBody::Bytes(body));
        self
    }

    /// Set the body content from a stream.
    pub fn streaming_body(mut self, body: impl Into<reqwest::Body>) -> Self {
        self.inner.body = Some(UploadPartBody::Stream(body.into()));
        self
    }

    /// Set the customer algorithm for SSE-C.
    pub fn ssec_customer_algorithm(mut self, algorithm: impl Into<String>) -> Self {
        self.inner.ssec_customer_algorithm = Some(algorithm.into());
        self
    }

    /// Set the customer key for SSE-C.
    pub fn ssec_customer_key(mut self, key: impl Into<String>) -> Self {
        self.inner.ssec_customer_key = Some(key.into());
        self
    }

    /// Set the customer key MD5 for SSE-C.
    pub fn ssec_customer_key_md5(mut self, md5: impl Into<String>) -> Self {
        self.inner.ssec_customer_key_md5 = Some(md5.into());
        self
    }

    /// Set the CRC64 ECMA checksum for data integrity verification.
    pub fn checksum_crc64ecma(mut self, checksum: impl Into<String>) -> Self {
        self.inner.checksum_crc64ecma = Some(checksum.into());
        self
    }

    /// Send the request.
    pub async fn send(self) -> Result<UploadPartOutput> {
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

        let mut params = HashMap::new();
        params.insert("partNumber".to_string(), self.inner.part_number.to_string());
        params.insert("uploadId".to_string(), upload_id.clone());

        let mut headers = reqwest::header::HeaderMap::new();

        // SSE-C headers
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

        // CRC64 checksum
        if let Some(ref checksum) = self.inner.checksum_crc64ecma {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(checksum) {
                headers.insert("x-obs-checksum-crc64ecma", value);
            }
        }

        // Handle body
        let body = self.inner.body.ok_or_else(|| ObsError::InvalidInput("body is required".to_string()))?;

        match body {
            UploadPartBody::Bytes(bytes) => {
                let resp = self
                    .client
                    .do_request(Method::PUT, Some(bucket), Some(key), Some(headers), Some(params), Some(bytes))
                    .await?;

                let status = resp.status();
                let headers_resp = resp.headers().clone();
                
                if !status.is_success() {
                    let text = resp.text().await?;
                    return Err(ObsError::service_error(status, &text));
                }

                let etag = headers_resp
                    .get("ETag")
                    .and_then(|v| v.to_str().ok())
                    .map(|s| s.trim_matches('"').to_string())
                    .unwrap_or_default();

                Ok(UploadPartOutput {
                    etag,
                    part_number: self.inner.part_number,
                })
            }
            UploadPartBody::Stream(stream) => {
                let resp = self
                    .client
                    .do_request_streaming(Method::PUT, Some(bucket), Some(key), Some(headers), Some(params), Some(stream))
                    .await?;

                let status = resp.status();
                let headers_resp = resp.headers().clone();
                
                if !status.is_success() {
                    let text = resp.text().await?;
                    return Err(ObsError::service_error(status, &text));
                }

                let etag = headers_resp
                    .get("ETag")
                    .and_then(|v| v.to_str().ok())
                    .map(|s| s.trim_matches('"').to_string())
                    .unwrap_or_default();

                Ok(UploadPartOutput {
                    etag,
                    part_number: self.inner.part_number,
                })
            }
        }
    }
}

impl std::fmt::Debug for UploadPartFluentBuilder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UploadPartFluentBuilder")
            .field("client", &self.client)
            .field("inner", &self.inner)
            .finish()
    }
}

/// Body type for UploadPart operation.
pub enum UploadPartBody {
    /// Bytes body.
    Bytes(Vec<u8>),
    /// Streaming body.
    Stream(reqwest::Body),
}

impl std::fmt::Debug for UploadPartBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UploadPartBody::Bytes(bytes) => f.debug_tuple("Bytes").field(&bytes.len()).finish(),
            UploadPartBody::Stream(_) => f.write_str("Stream(..)"),
        }
    }
}

/// Input for the UploadPart operation.
#[derive(Debug, Default)]
pub struct UploadPartInput {
    bucket: String,
    key: String,
    upload_id: String,
    part_number: i32,
    body: Option<UploadPartBody>,
    ssec_customer_algorithm: Option<String>,
    ssec_customer_key: Option<String>,
    ssec_customer_key_md5: Option<String>,
    checksum_crc64ecma: Option<String>,
}

/// Output for the UploadPart operation.
#[derive(Debug, Clone)]
pub struct UploadPartOutput {
    etag: String,
    part_number: i32,
}

impl UploadPartOutput {
    /// Get the ETag of the uploaded part.
    pub fn etag(&self) -> &str {
        &self.etag
    }

    /// Get the part number.
    pub fn part_number(&self) -> i32 {
        self.part_number
    }
}