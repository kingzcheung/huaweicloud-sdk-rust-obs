//! Complete Multipart Upload operation.

use std::collections::HashMap;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::{ObsError, Result};

/// Fluent builder for the CompleteMultipartUpload operation.
///
/// This operation completes a multipart upload by assembling previously uploaded parts.
#[derive(Debug, Clone)]
pub struct CompleteMultipartUploadFluentBuilder {
    client: Client,
    inner: CompleteMultipartUploadInput,
}

impl CompleteMultipartUploadFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: CompleteMultipartUploadInput::default(),
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

    /// Add a part to the completion list.
    pub fn part(mut self, part_number: i32, etag: impl Into<String>) -> Self {
        self.inner.parts.push(CompletedPart {
            part_number,
            etag: etag.into(),
        });
        self
    }

    /// Set all parts at once.
    pub fn parts(mut self, parts: Vec<CompletedPart>) -> Self {
        self.inner.parts = parts;
        self
    }

    /// Set the CRC64 ECMA checksum for data integrity verification.
    pub fn checksum_crc64ecma(mut self, checksum: impl Into<String>) -> Self {
        self.inner.checksum_crc64ecma = Some(checksum.into());
        self
    }

    /// Set the encoding type.
    pub fn encoding_type(mut self, encoding_type: impl Into<String>) -> Self {
        self.inner.encoding_type = Some(encoding_type.into());
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<CompleteMultipartUploadOutput> {
        let bucket = &self.inner.bucket;
        let key = &self.inner.key;
        let upload_id = &self.inner.upload_id;

        if bucket.is_empty() {
            return Err(ObsError::InvalidInput(
                "bucket name is required".to_string(),
            ));
        }
        if key.is_empty() {
            return Err(ObsError::InvalidInput("object key is required".to_string()));
        }
        if upload_id.is_empty() {
            return Err(ObsError::InvalidInput("upload ID is required".to_string()));
        }
        if self.inner.parts.is_empty() {
            return Err(ObsError::InvalidInput(
                "at least one part is required".to_string(),
            ));
        }

        // Sort parts by part number
        let mut sorted_parts = self.inner.parts.clone();
        sorted_parts.sort_by_key(|p| p.part_number);

        let mut params = HashMap::new();
        params.insert("uploadId".to_string(), upload_id.clone());

        if let Some(ref encoding_type) = self.inner.encoding_type {
            params.insert("encoding-type".to_string(), encoding_type.clone());
        }

        let mut headers = reqwest::header::HeaderMap::new();

        // CRC64 checksum
        if let Some(ref checksum) = self.inner.checksum_crc64ecma {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(checksum) {
                headers.insert("x-obs-checksum-crc64ecma", value);
            }
        }

        // Build XML body
        let xml_parts: Vec<String> = sorted_parts
            .iter()
            .map(|p| {
                format!(
                    "<Part><PartNumber>{}</PartNumber><ETag>{}</ETag></Part>",
                    p.part_number, p.etag
                )
            })
            .collect();

        let body = format!(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?><CompleteMultipartUpload>{}</CompleteMultipartUpload>",
            xml_parts.join("")
        );

        let resp = self
            .client
            .do_request(
                Method::POST,
                Some(bucket),
                Some(key),
                Some(headers),
                Some(params),
                Some(body.into_bytes()),
            )
            .await?;

        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            return Err(ObsError::service_error(status, &text));
        }

        let result: CompleteMultipartUploadResultXml = crate::xml_utils::from_xml(&text)?;

        Ok(CompleteMultipartUploadOutput::from(result))
    }
}

/// A completed part information.
#[derive(Debug, Clone)]
pub struct CompletedPart {
    /// Part number.
    pub part_number: i32,
    /// ETag of the part.
    pub etag: String,
}

impl CompletedPart {
    /// Create a new completed part.
    pub fn new(part_number: i32, etag: impl Into<String>) -> Self {
        Self {
            part_number,
            etag: etag.into(),
        }
    }
}

/// Input for the CompleteMultipartUpload operation.
#[derive(Debug, Clone, Default)]
pub struct CompleteMultipartUploadInput {
    bucket: String,
    key: String,
    upload_id: String,
    parts: Vec<CompletedPart>,
    checksum_crc64ecma: Option<String>,
    encoding_type: Option<String>,
}

/// Output for the CompleteMultipartUpload operation.
#[derive(Debug, Clone)]
pub struct CompleteMultipartUploadOutput {
    location: String,
    bucket: String,
    key: String,
    etag: String,
    encoding_type: Option<String>,
}

impl CompleteMultipartUploadOutput {
    /// Get the location.
    pub fn location(&self) -> &str {
        &self.location
    }

    /// Get the bucket name.
    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    /// Get the object key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Get the ETag.
    pub fn etag(&self) -> &str {
        &self.etag
    }

    /// Get the encoding type.
    pub fn encoding_type(&self) -> Option<&str> {
        self.encoding_type.as_deref()
    }
}

impl From<CompleteMultipartUploadResultXml> for CompleteMultipartUploadOutput {
    fn from(value: CompleteMultipartUploadResultXml) -> Self {
        Self {
            location: value.location,
            bucket: value.bucket,
            key: value.key,
            etag: value.etag,
            encoding_type: value.encoding_type,
        }
    }
}

// Internal XML response type for CompleteMultipartUpload
#[derive(Debug, Serialize, Deserialize)]
struct CompleteMultipartUploadResultXml {
    #[serde(rename = "Location")]
    location: String,
    #[serde(rename = "Bucket")]
    bucket: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "ETag")]
    etag: String,
    #[serde(rename = "EncodingType")]
    encoding_type: Option<String>,
}
