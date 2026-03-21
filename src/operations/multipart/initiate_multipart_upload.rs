//! Initiate Multipart Upload operation.

use std::collections::HashMap;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::{ObsError, Result};

/// Fluent builder for the InitiateMultipartUpload operation.
///
/// This operation initiates a multipart upload and returns an upload ID.
#[derive(Debug, Clone)]
pub struct InitiateMultipartUploadFluentBuilder {
    client: Client,
    inner: InitiateMultipartUploadInput,
}

impl InitiateMultipartUploadFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: InitiateMultipartUploadInput::default(),
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

    /// Set the ACL (Access Control List).
    ///
    /// Possible values: private, public-read, public-read-write.
    pub fn acl(mut self, acl: impl Into<String>) -> Self {
        self.inner.acl = Some(acl.into());
        self
    }

    /// Set the storage class.
    ///
    /// Possible values: STANDARD, WARM, COLD, DEEP_ARCHIVE.
    pub fn storage_class(mut self, storage_class: impl Into<String>) -> Self {
        self.inner.storage_class = Some(storage_class.into());
        self
    }

    /// Set the grant-read ACL header.
    ///
    /// Grants read permission to the specified domain IDs.
    pub fn grant_read(mut self, grant_read: impl Into<String>) -> Self {
        self.inner.grant_read = Some(grant_read.into());
        self
    }

    /// Set the grant-read-acp ACL header.
    ///
    /// Grants read ACL permission to the specified domain IDs.
    pub fn grant_read_acp(mut self, grant_read_acp: impl Into<String>) -> Self {
        self.inner.grant_read_acp = Some(grant_read_acp.into());
        self
    }

    /// Set the grant-write-acp ACL header.
    ///
    /// Grants write ACL permission to the specified domain IDs.
    pub fn grant_write_acp(mut self, grant_write_acp: impl Into<String>) -> Self {
        self.inner.grant_write_acp = Some(grant_write_acp.into());
        self
    }

    /// Set the grant-full-control ACL header.
    ///
    /// Grants full control permission to the specified domain IDs.
    pub fn grant_full_control(mut self, grant_full_control: impl Into<String>) -> Self {
        self.inner.grant_full_control = Some(grant_full_control.into());
        self
    }

    /// Set the website redirect location.
    pub fn website_redirect_location(mut self, location: impl Into<String>) -> Self {
        self.inner.website_redirect_location = Some(location.into());
        self
    }

    /// Set the server-side encryption type.
    ///
    /// Possible values: kms, AES256.
    pub fn server_side_encryption(mut self, sse: impl Into<String>) -> Self {
        self.inner.server_side_encryption = Some(sse.into());
        self
    }

    /// Set the server-side data encryption algorithm.
    ///
    /// Possible values: AES256, SM4.
    pub fn server_side_data_encryption(mut self, algorithm: impl Into<String>) -> Self {
        self.inner.server_side_data_encryption = Some(algorithm.into());
        self
    }

    /// Set the KMS key ID for server-side encryption.
    pub fn ssekms_key_id(mut self, key_id: impl Into<String>) -> Self {
        self.inner.ssekms_key_id = Some(key_id.into());
        self
    }

    /// Set whether bucket key is enabled for SSE-KMS.
    pub fn bucket_key_enabled(mut self, enabled: bool) -> Self {
        self.inner.bucket_key_enabled = Some(enabled);
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

    /// Set the object expiration time in days.
    pub fn expires(mut self, days: i32) -> Self {
        self.inner.expires = Some(days);
        self
    }

    /// Set the object tagging.
    ///
    /// Format: TagA=A&TagB&TagC
    pub fn tagging(mut self, tagging: impl Into<String>) -> Self {
        self.inner.tagging = Some(tagging.into());
        self
    }

    /// Set the object lock mode.
    ///
    /// Possible value: COMPLIANCE.
    pub fn object_lock_mode(mut self, mode: impl Into<String>) -> Self {
        self.inner.object_lock_mode = Some(mode.into());
        self
    }

    /// Set the object lock retain until date.
    ///
    /// Format: UTC time in ISO 8601 format.
    pub fn object_lock_retain_until_date(mut self, date: impl Into<String>) -> Self {
        self.inner.object_lock_retain_until_date = Some(date.into());
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

    /// Add a custom metadata header.
    ///
    /// The key should not include the "x-obs-meta-" prefix.
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.inner.metadata.insert(key.into(), value.into());
        self
    }

    /// Set the encoding type for the response.
    pub fn encoding_type(mut self, encoding_type: impl Into<String>) -> Self {
        self.inner.encoding_type = Some(encoding_type.into());
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<InitiateMultipartUploadOutput> {
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
        params.insert("uploads".to_string(), String::new());

        if let Some(ref encoding_type) = self.inner.encoding_type {
            params.insert("encoding-type".to_string(), encoding_type.clone());
        }

        let mut headers = reqwest::header::HeaderMap::new();

        // ACL headers
        if let Some(ref acl) = self.inner.acl {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(acl) {
                headers.insert("x-obs-acl", value);
            }
        }

        if let Some(ref grant_read) = self.inner.grant_read {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(grant_read) {
                headers.insert("x-obs-grant-read", value);
            }
        }

        if let Some(ref grant_read_acp) = self.inner.grant_read_acp {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(grant_read_acp) {
                headers.insert("x-obs-grant-read-acp", value);
            }
        }

        if let Some(ref grant_write_acp) = self.inner.grant_write_acp {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(grant_write_acp) {
                headers.insert("x-obs-grant-write-acp", value);
            }
        }

        if let Some(ref grant_full_control) = self.inner.grant_full_control {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(grant_full_control) {
                headers.insert("x-obs-grant-full-control", value);
            }
        }

        // Storage class
        if let Some(ref storage_class) = self.inner.storage_class {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(storage_class) {
                headers.insert("x-obs-storage-class", value);
            }
        }

        // Website redirect
        if let Some(ref location) = self.inner.website_redirect_location {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(location) {
                headers.insert("x-obs-website-redirect-location", value);
            }
        }

        // Server-side encryption
        if let Some(ref sse) = self.inner.server_side_encryption {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(sse) {
                headers.insert("x-obs-server-side-encryption", value);
            }
        }

        if let Some(ref algorithm) = self.inner.server_side_data_encryption {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(algorithm) {
                headers.insert("x-obs-server-side-data-encryption", value);
            }
        }

        if let Some(ref key_id) = self.inner.ssekms_key_id {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(key_id) {
                headers.insert("x-obs-server-side-encryption-kms-key-id", value);
            }
        }

        if let Some(enabled) = self.inner.bucket_key_enabled {
            if let Ok(value) =
                reqwest::header::HeaderValue::from_str(if enabled { "true" } else { "false" })
            {
                headers.insert("x-obs-server-side-encryption-bucket-key-enabled", value);
            }
        }

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

        // Expiration
        if let Some(days) = self.inner.expires {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(&days.to_string()) {
                headers.insert("x-obs-expires", value);
            }
        }

        // Tagging
        if let Some(ref tagging) = self.inner.tagging {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(tagging) {
                headers.insert("x-obs-tagging", value);
            }
        }

        // Object lock
        if let Some(ref mode) = self.inner.object_lock_mode {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(mode) {
                headers.insert("x-obs-object-lock-mode", value);
            }
        }

        if let Some(ref date) = self.inner.object_lock_retain_until_date {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(date) {
                headers.insert("x-obs-object-lock-retain-until-date", value);
            }
        }

        // Content headers
        if let Some(ref content_type) = self.inner.content_type {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(content_type) {
                headers.insert("Content-Type", value);
            }
        }

        if let Some(ref content_encoding) = self.inner.content_encoding {
            if let Ok(value) = reqwest::header::HeaderValue::from_str(content_encoding) {
                headers.insert("Content-Encoding", value);
            }
        }

        // Custom metadata
        for (k, v) in &self.inner.metadata {
            let header_name = format!("x-obs-meta-{}", k);
            if let Ok(name) = reqwest::header::HeaderName::try_from(header_name) {
                if let Ok(value) = reqwest::header::HeaderValue::from_str(v) {
                    headers.insert(name, value);
                }
            }
        }

        let resp = self
            .client
            .do_request(
                Method::POST,
                Some(bucket),
                Some(key),
                Some(headers),
                Some(params),
                None,
            )
            .await?;

        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            return Err(ObsError::service_error(status, &text));
        }

        let result: InitiateMultipartUploadResultXml = crate::xml_utils::from_xml(&text)?;

        Ok(InitiateMultipartUploadOutput::from(result))
    }
}

/// Input for the InitiateMultipartUpload operation.
#[derive(Debug, Clone, Default)]
pub struct InitiateMultipartUploadInput {
    bucket: String,
    key: String,
    acl: Option<String>,
    storage_class: Option<String>,
    grant_read: Option<String>,
    grant_read_acp: Option<String>,
    grant_write_acp: Option<String>,
    grant_full_control: Option<String>,
    website_redirect_location: Option<String>,
    server_side_encryption: Option<String>,
    server_side_data_encryption: Option<String>,
    ssekms_key_id: Option<String>,
    bucket_key_enabled: Option<bool>,
    ssec_customer_algorithm: Option<String>,
    ssec_customer_key: Option<String>,
    ssec_customer_key_md5: Option<String>,
    expires: Option<i32>,
    tagging: Option<String>,
    object_lock_mode: Option<String>,
    object_lock_retain_until_date: Option<String>,
    content_type: Option<String>,
    content_encoding: Option<String>,
    metadata: std::collections::HashMap<String, String>,
    encoding_type: Option<String>,
}

/// Output for the InitiateMultipartUpload operation.
#[derive(Debug, Clone)]
pub struct InitiateMultipartUploadOutput {
    bucket: String,
    key: String,
    upload_id: String,
    encoding_type: Option<String>,
}

impl InitiateMultipartUploadOutput {
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

    /// Get the encoding type.
    pub fn encoding_type(&self) -> Option<&str> {
        self.encoding_type.as_deref()
    }
}

impl From<InitiateMultipartUploadResultXml> for InitiateMultipartUploadOutput {
    fn from(value: InitiateMultipartUploadResultXml) -> Self {
        Self {
            bucket: value.bucket,
            key: value.key,
            upload_id: value.upload_id,
            encoding_type: value.encoding_type,
        }
    }
}

// Internal XML response type for InitiateMultipartUpload
#[derive(Debug, Serialize, Deserialize)]
struct InitiateMultipartUploadResultXml {
    #[serde(rename = "Bucket")]
    bucket: String,
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "UploadId")]
    upload_id: String,
    #[serde(rename = "EncodingType")]
    encoding_type: Option<String>,
}
