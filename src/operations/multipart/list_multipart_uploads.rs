//! List Multipart Uploads operation.

use std::collections::HashMap;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::{ObsError, Result};

/// Fluent builder for the ListMultipartUploads operation.
///
/// This operation lists in-progress multipart uploads.
#[derive(Debug, Clone)]
pub struct ListMultipartUploadsFluentBuilder {
    client: Client,
    inner: ListMultipartUploadsInput,
}

impl ListMultipartUploadsFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: ListMultipartUploadsInput::default(),
        }
    }

    /// Set the bucket name.
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.inner.bucket = bucket.into();
        self
    }

    /// Set the delimiter.
    ///
    /// The delimiter that groups object names.
    pub fn delimiter(mut self, delimiter: impl Into<String>) -> Self {
        self.inner.delimiter = Some(delimiter.into());
        self
    }

    /// Set the prefix.
    ///
    /// Lists only object names that begin with this prefix.
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.inner.prefix = Some(prefix.into());
        self
    }

    /// Set the max uploads.
    ///
    /// Maximum number of multipart uploads to return. Range: [1, 1000].
    pub fn max_uploads(mut self, max_uploads: i32) -> Self {
        self.inner.max_uploads = Some(max_uploads);
        self
    }

    /// Set the key marker.
    ///
    /// Specifies the object key after which listing should begin.
    pub fn key_marker(mut self, key_marker: impl Into<String>) -> Self {
        self.inner.key_marker = Some(key_marker.into());
        self
    }

    /// Set the upload ID marker.
    ///
    /// Specifies the upload ID after which listing should begin.
    pub fn upload_id_marker(mut self, upload_id_marker: impl Into<String>) -> Self {
        self.inner.upload_id_marker = Some(upload_id_marker.into());
        self
    }

    /// Set the encoding type.
    ///
    /// Encoding type for the response. Currently only "url" is supported.
    pub fn encoding_type(mut self, encoding_type: impl Into<String>) -> Self {
        self.inner.encoding_type = Some(encoding_type.into());
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<ListMultipartUploadsOutput> {
        let bucket = &self.inner.bucket;
        if bucket.is_empty() {
            return Err(ObsError::InvalidInput(
                "bucket name is required".to_string(),
            ));
        }

        let mut params = HashMap::new();
        params.insert("uploads".to_string(), String::new());

        if let Some(ref delimiter) = self.inner.delimiter {
            params.insert("delimiter".to_string(), delimiter.clone());
        }

        if let Some(ref prefix) = self.inner.prefix {
            params.insert("prefix".to_string(), prefix.clone());
        }

        if let Some(max_uploads) = self.inner.max_uploads {
            params.insert("max-uploads".to_string(), max_uploads.to_string());
        }

        if let Some(ref key_marker) = self.inner.key_marker {
            params.insert("key-marker".to_string(), key_marker.clone());
        }

        if let Some(ref upload_id_marker) = self.inner.upload_id_marker {
            params.insert("upload-id-marker".to_string(), upload_id_marker.clone());
        }

        if let Some(ref encoding_type) = self.inner.encoding_type {
            params.insert("encoding-type".to_string(), encoding_type.clone());
        }

        let resp = self
            .client
            .do_request(Method::GET, Some(bucket), None, None, Some(params), None)
            .await?;

        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            return Err(ObsError::service_error(status, &text));
        }

        let result: ListMultipartUploadsResultXml = crate::xml_utils::from_xml(&text)?;

        Ok(ListMultipartUploadsOutput::from(result))
    }
}

/// Input for the ListMultipartUploads operation.
#[derive(Debug, Clone, Default)]
pub struct ListMultipartUploadsInput {
    bucket: String,
    delimiter: Option<String>,
    prefix: Option<String>,
    max_uploads: Option<i32>,
    key_marker: Option<String>,
    upload_id_marker: Option<String>,
    encoding_type: Option<String>,
}

/// Output for the ListMultipartUploads operation.
#[derive(Debug, Clone)]
pub struct ListMultipartUploadsOutput {
    bucket: String,
    key_marker: Option<String>,
    upload_id_marker: Option<String>,
    next_key_marker: Option<String>,
    next_upload_id_marker: Option<String>,
    max_uploads: i32,
    is_truncated: bool,
    uploads: Vec<MultipartUpload>,
    common_prefixes: Vec<String>,
    delimiter: Option<String>,
    prefix: Option<String>,
    encoding_type: Option<String>,
}

impl ListMultipartUploadsOutput {
    /// Get the bucket name.
    pub fn bucket(&self) -> &str {
        &self.bucket
    }

    /// Get the key marker.
    pub fn key_marker(&self) -> Option<&str> {
        self.key_marker.as_deref()
    }

    /// Get the upload ID marker.
    pub fn upload_id_marker(&self) -> Option<&str> {
        self.upload_id_marker.as_deref()
    }

    /// Get the next key marker.
    pub fn next_key_marker(&self) -> Option<&str> {
        self.next_key_marker.as_deref()
    }

    /// Get the next upload ID marker.
    pub fn next_upload_id_marker(&self) -> Option<&str> {
        self.next_upload_id_marker.as_deref()
    }

    /// Get the max uploads.
    pub fn max_uploads(&self) -> i32 {
        self.max_uploads
    }

    /// Check if the result is truncated.
    pub fn is_truncated(&self) -> bool {
        self.is_truncated
    }

    /// Get the list of multipart uploads.
    pub fn uploads(&self) -> &[MultipartUpload] {
        &self.uploads
    }

    /// Get the common prefixes.
    pub fn common_prefixes(&self) -> &[String] {
        &self.common_prefixes
    }

    /// Get the delimiter.
    pub fn delimiter(&self) -> Option<&str> {
        self.delimiter.as_deref()
    }

    /// Get the prefix.
    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
    }

    /// Get the encoding type.
    pub fn encoding_type(&self) -> Option<&str> {
        self.encoding_type.as_deref()
    }
}

impl From<ListMultipartUploadsResultXml> for ListMultipartUploadsOutput {
    fn from(value: ListMultipartUploadsResultXml) -> Self {
        Self {
            bucket: value.bucket,
            key_marker: value.key_marker,
            upload_id_marker: value.upload_id_marker,
            next_key_marker: value.next_key_marker,
            next_upload_id_marker: value.next_upload_id_marker,
            max_uploads: value.max_uploads.parse().unwrap_or(1000),
            is_truncated: value.is_truncated == "true",
            uploads: value.uploads.unwrap_or_default(),
            common_prefixes: value
                .common_prefixes
                .unwrap_or_default()
                .into_iter()
                .map(|p| p.prefix)
                .collect(),
            delimiter: value.delimiter,
            prefix: value.prefix,
            encoding_type: value.encoding_type,
        }
    }
}

/// Information about a multipart upload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipartUpload {
    /// Object key.
    #[serde(rename = "Key")]
    key: String,
    /// Upload ID.
    #[serde(rename = "UploadId")]
    upload_id: String,
    /// Initiator information.
    #[serde(rename = "Initiator")]
    initiator: MultipartInitiator,
    /// Owner information.
    #[serde(rename = "Owner")]
    owner: MultipartOwner,
    /// Storage class.
    #[serde(rename = "StorageClass")]
    storage_class: String,
    /// Initiated timestamp.
    #[serde(rename = "Initiated")]
    initiated: String,
}

impl MultipartUpload {
    /// Get the object key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Get the upload ID.
    pub fn upload_id(&self) -> &str {
        &self.upload_id
    }

    /// Get the initiator.
    pub fn initiator(&self) -> &MultipartInitiator {
        &self.initiator
    }

    /// Get the owner.
    pub fn owner(&self) -> &MultipartOwner {
        &self.owner
    }

    /// Get the storage class.
    pub fn storage_class(&self) -> &str {
        &self.storage_class
    }

    /// Get the initiated timestamp.
    pub fn initiated(&self) -> &str {
        &self.initiated
    }
}

/// Initiator information for multipart upload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipartInitiator {
    /// Initiator ID (format: domainID/{domainId}:userID/{userId}).
    #[serde(rename = "ID")]
    id: String,
}

impl MultipartInitiator {
    /// Get the initiator ID.
    pub fn id(&self) -> &str {
        &self.id
    }
}

/// Owner information for multipart upload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultipartOwner {
    /// Owner domain ID.
    #[serde(rename = "ID")]
    id: String,
}

impl MultipartOwner {
    /// Get the owner ID.
    pub fn id(&self) -> &str {
        &self.id
    }
}

// Internal XML response types

#[derive(Debug, Serialize, Deserialize)]
struct ListMultipartUploadsResultXml {
    #[serde(rename = "Bucket")]
    bucket: String,
    #[serde(rename = "KeyMarker")]
    key_marker: Option<String>,
    #[serde(rename = "UploadIdMarker")]
    upload_id_marker: Option<String>,
    #[serde(rename = "NextKeyMarker")]
    next_key_marker: Option<String>,
    #[serde(rename = "NextUploadIdMarker")]
    next_upload_id_marker: Option<String>,
    #[serde(rename = "MaxUploads")]
    max_uploads: String,
    #[serde(rename = "IsTruncated")]
    is_truncated: String,
    #[serde(rename = "EncodingType")]
    encoding_type: Option<String>,
    #[serde(rename = "Delimiter")]
    delimiter: Option<String>,
    #[serde(rename = "Prefix")]
    prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Upload")]
    uploads: Option<Vec<MultipartUpload>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "CommonPrefixes")]
    common_prefixes: Option<Vec<CommonPrefix>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommonPrefix {
    #[serde(rename = "Prefix")]
    prefix: String,
}
