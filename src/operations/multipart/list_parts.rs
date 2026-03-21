//! List Parts operation.

use std::collections::HashMap;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::{ObsError, Result};

/// Fluent builder for the ListParts operation.
///
/// This operation lists the parts that have been uploaded for a specific multipart upload.
#[derive(Debug, Clone)]
pub struct ListPartsFluentBuilder {
    client: Client,
    inner: ListPartsInput,
}

impl ListPartsFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: ListPartsInput::default(),
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

    /// Set the max parts to return.
    ///
    /// Range: [1, 1000].
    pub fn max_parts(mut self, max_parts: i32) -> Self {
        self.inner.max_parts = Some(max_parts);
        self
    }

    /// Set the part number marker.
    ///
    /// Only parts with part number greater than this value will be listed.
    pub fn part_number_marker(mut self, marker: i32) -> Self {
        self.inner.part_number_marker = Some(marker);
        self
    }

    /// Set the encoding type.
    pub fn encoding_type(mut self, encoding_type: impl Into<String>) -> Self {
        self.inner.encoding_type = Some(encoding_type.into());
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<ListPartsOutput> {
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

        if let Some(max_parts) = self.inner.max_parts {
            params.insert("max-parts".to_string(), max_parts.to_string());
        }

        if let Some(marker) = self.inner.part_number_marker {
            params.insert("part-number-marker".to_string(), marker.to_string());
        }

        if let Some(ref encoding_type) = self.inner.encoding_type {
            params.insert("encoding-type".to_string(), encoding_type.clone());
        }

        let resp = self
            .client
            .do_request(Method::GET, Some(bucket), Some(key), None, Some(params), None)
            .await?;

        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            return Err(ObsError::service_error(status, &text));
        }

        let result: ListPartsResultXml =
            crate::xml_utils::from_xml(&text)?;

        Ok(ListPartsOutput::from(result))
    }
}

/// Input for the ListParts operation.
#[derive(Debug, Clone, Default)]
pub struct ListPartsInput {
    bucket: String,
    key: String,
    upload_id: String,
    max_parts: Option<i32>,
    part_number_marker: Option<i32>,
    encoding_type: Option<String>,
}

/// Output for the ListParts operation.
#[derive(Debug, Clone)]
pub struct ListPartsOutput {
    bucket: String,
    key: String,
    upload_id: String,
    initiator: ListPartsInitiator,
    owner: ListPartsOwner,
    storage_class: String,
    part_number_marker: i32,
    next_part_number_marker: Option<i32>,
    max_parts: i32,
    is_truncated: bool,
    parts: Vec<PartInfo>,
    encoding_type: Option<String>,
}

impl ListPartsOutput {
    /// Get the bucket name.
    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    /// Get the object key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Get the upload ID.
    pub fn upload_id(&self) -> &str {
        &self.upload_id
    }

    /// Get the initiator.
    pub fn initiator(&self) -> &ListPartsInitiator {
        &self.initiator
    }

    /// Get the owner.
    pub fn owner(&self) -> &ListPartsOwner {
        &self.owner
    }

    /// Get the storage class.
    pub fn storage_class(&self) -> &str {
        &self.storage_class
    }

    /// Get the part number marker.
    pub fn part_number_marker(&self) -> i32 {
        self.part_number_marker
    }

    /// Get the next part number marker.
    pub fn next_part_number_marker(&self) -> Option<i32> {
        self.next_part_number_marker
    }

    /// Get the max parts.
    pub fn max_parts(&self) -> i32 {
        self.max_parts
    }

    /// Check if the result is truncated.
    pub fn is_truncated(&self) -> bool {
        self.is_truncated
    }

    /// Get the list of parts.
    pub fn parts(&self) -> &[PartInfo] {
        &self.parts
    }

    /// Get the encoding type.
    pub fn encoding_type(&self) -> Option<&str> {
        self.encoding_type.as_deref()
    }
}

impl From<ListPartsResultXml> for ListPartsOutput {
    fn from(value: ListPartsResultXml) -> Self {
        Self {
            bucket: value.bucket,
            key: value.key,
            upload_id: value.upload_id,
            initiator: value.initiator,
            owner: value.owner,
            storage_class: value.storage_class,
            part_number_marker: value.part_number_marker.parse().unwrap_or(0),
            next_part_number_marker: value.next_part_number_marker.and_then(|s| s.parse().ok()),
            max_parts: value.max_parts.parse().unwrap_or(1000),
            is_truncated: value.is_truncated == "true",
            parts: value.parts.unwrap_or_default(),
            encoding_type: value.encoding_type,
        }
    }
}

/// Initiator information for ListParts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPartsInitiator {
    /// Initiator ID.
    #[serde(rename = "ID")]
    id: String,
}

impl ListPartsInitiator {
    /// Get the initiator ID.
    pub fn id(&self) -> &str {
        &self.id
    }
}

/// Owner information for ListParts.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListPartsOwner {
    /// Owner ID.
    #[serde(rename = "ID")]
    id: String,
}

impl ListPartsOwner {
    /// Get the owner ID.
    pub fn id(&self) -> &str {
        &self.id
    }
}

/// Part information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartInfo {
    /// Part number.
    #[serde(rename = "PartNumber")]
    part_number: i32,
    /// Last modified time.
    #[serde(rename = "LastModified")]
    last_modified: String,
    /// ETag.
    #[serde(rename = "ETag")]
    etag: String,
    /// Size in bytes.
    #[serde(rename = "Size")]
    size: i64,
}

impl PartInfo {
    /// Get the part number.
    pub fn part_number(&self) -> i32 {
        self.part_number
    }

    /// Get the last modified time.
    pub fn last_modified(&self) -> &str {
        &self.last_modified
    }

    /// Get the ETag.
    pub fn etag(&self) -> &str {
        &self.etag
    }

    /// Get the size in bytes.
    pub fn size(&self) -> i64 {
        self.size
    }
}

// Internal XML response type for ListParts
#[derive(Debug, Serialize, Deserialize)]
struct ListPartsResultXml {
    #[serde(rename = "Bucket")]
    bucket: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "UploadId")]
    upload_id: String,
    #[serde(rename = "Initiator")]
    initiator: ListPartsInitiator,
    #[serde(rename = "Owner")]
    owner: ListPartsOwner,
    #[serde(rename = "StorageClass")]
    storage_class: String,
    #[serde(rename = "PartNumberMarker")]
    part_number_marker: String,
    #[serde(rename = "NextPartNumberMarker")]
    next_part_number_marker: Option<String>,
    #[serde(rename = "MaxParts")]
    max_parts: String,
    #[serde(rename = "IsTruncated")]
    is_truncated: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Part")]
    parts: Option<Vec<PartInfo>>,
    #[serde(rename = "EncodingType")]
    encoding_type: Option<String>,
}