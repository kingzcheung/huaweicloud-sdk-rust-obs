//! Object operations - AWS SDK style fluent builders.

use std::collections::HashMap;

use base64::{engine::general_purpose, Engine};
use md5::{Digest, Md5};
use reqwest::{
    header::{HeaderMap, HeaderValue},
    Method,
};
use serde::{Deserialize, Serialize};
use urlencoding::encode;

use crate::client::Client;
use crate::error::{ObsError, Result};

// ========================================
// Put Object
// ========================================

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
                let header_name: reqwest::header::HeaderName = format!("x-obs-meta-{}", k).parse().unwrap();
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

// ========================================
// Get Object
// ========================================

/// Fluent builder for the GetObject operation.
#[derive(Debug, Clone)]
pub struct GetObjectFluentBuilder {
    client: Client,
    inner: GetObjectInput,
}

impl GetObjectFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: GetObjectInput::default(),
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

    /// Set the range.
    pub fn range(mut self, range: impl Into<String>) -> Self {
        self.inner.range = Some(range.into());
        self
    }

    /// Set the version ID.
    pub fn version_id(mut self, version_id: impl Into<String>) -> Self {
        self.inner.version_id = Some(version_id.into());
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<GetObjectOutput> {
        let bucket = &self.inner.bucket;
        let key = &self.inner.key;

        if bucket.is_empty() {
            return Err(ObsError::InvalidInput("bucket name is required".to_string()));
        }
        if key.is_empty() {
            return Err(ObsError::InvalidInput("object key is required".to_string()));
        }

        let mut headers = HeaderMap::new();
        if let Some(ref range) = self.inner.range {
            headers.insert("Range", HeaderValue::from_str(range).unwrap());
        }

        let mut params = HashMap::new();
        if let Some(ref version_id) = self.inner.version_id {
            params.insert("versionId".to_string(), version_id.clone());
        }

        let resp = self
            .client
            .do_request(Method::GET, Some(bucket), Some(key), Some(headers), Some(params), None)
            .await?;

        let status = resp.status();
        let response_headers = resp.headers().clone();

        if !status.is_success() {
            let text = resp.text().await?;
            return Err(ObsError::service_error(status, &text));
        }

        let body = resp.bytes().await?;

        let content_type = response_headers
            .get("Content-Type")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let content_length = response_headers
            .get("Content-Length")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok());

        let etag = response_headers
            .get("ETag")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.trim_matches('"').to_string());

        let last_modified = response_headers
            .get("Last-Modified")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        Ok(GetObjectOutput {
            body,
            content_type,
            content_length,
            etag,
            last_modified,
        })
    }
}

/// Input for the GetObject operation.
#[derive(Debug, Clone, Default)]
pub struct GetObjectInput {
    bucket: String,
    key: String,
    range: Option<String>,
    version_id: Option<String>,
}

/// Output for the GetObject operation.
#[derive(Debug)]
pub struct GetObjectOutput {
    body: bytes::Bytes,
    content_type: Option<String>,
    content_length: Option<u64>,
    etag: Option<String>,
    last_modified: Option<String>,
}

impl GetObjectOutput {
    /// Get the body.
    pub fn body(&self) -> &bytes::Bytes {
        &self.body
    }

    /// Get the body as bytes.
    pub fn body_bytes(&self) -> &[u8] {
        &self.body
    }

    /// Consume the output and get the body.
    pub fn into_body(self) -> bytes::Bytes {
        self.body
    }

    /// Get the content type.
    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }

    /// Get the content length.
    pub fn content_length(&self) -> Option<u64> {
        self.content_length
    }

    /// Get the ETag.
    pub fn etag(&self) -> Option<&str> {
        self.etag.as_deref()
    }

    /// Get the last modified time.
    pub fn last_modified(&self) -> Option<&str> {
        self.last_modified.as_deref()
    }
}

// ========================================
// Delete Object
// ========================================

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
            return Err(ObsError::InvalidInput("bucket name is required".to_string()));
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
            .do_request(Method::DELETE, Some(bucket), Some(key), None, Some(params), None)
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

// ========================================
// Delete Objects
// ========================================

/// Fluent builder for the DeleteObjects operation.
#[derive(Debug, Clone)]
pub struct DeleteObjectsFluentBuilder {
    client: Client,
    inner: DeleteObjectsInput,
}

impl DeleteObjectsFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: DeleteObjectsInput::default(),
        }
    }

    /// Set the bucket name.
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.inner.bucket = bucket.into();
        self
    }

    /// Add an object to delete.
    pub fn key(mut self, key: impl Into<String>) -> Self {
        self.inner.keys.push(DeleteObject {
            key: key.into(),
            version_id: None,
        });
        self
    }

    /// Add multiple objects to delete.
    pub fn keys(mut self, keys: Vec<String>) -> Self {
        for key in keys {
            self.inner.keys.push(DeleteObject {
                key,
                version_id: None,
            });
        }
        self
    }

    /// Set quiet mode.
    pub fn quiet(mut self, quiet: bool) -> Self {
        self.inner.quiet = quiet;
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<DeleteObjectsOutput> {
        let bucket = &self.inner.bucket;

        if bucket.is_empty() {
            return Err(ObsError::InvalidInput("bucket name is required".to_string()));
        }
        if self.inner.keys.is_empty() {
            return Err(ObsError::InvalidInput("at least one object key is required".to_string()));
        }

        let mut params = HashMap::new();
        params.insert("delete".to_string(), String::new());

        let delete = DeleteRequest {
            quiet: self.inner.quiet,
            object: self.inner.keys.clone(),
        };

        let body = serde_xml_rs::to_string(&delete)
            .map_err(|e| ObsError::Serialization(e.to_string()))?;

        let mut hasher = Md5::new();
        hasher.update(body.as_bytes());
        let result = hasher.finalize();
        let md5_value = general_purpose::STANDARD.encode(result);

        let mut headers = HeaderMap::new();
        headers.insert("Content-MD5", HeaderValue::from_str(&md5_value).unwrap());
        headers.insert(
            "Content-Length",
            HeaderValue::from_str(&body.len().to_string()).unwrap(),
        );

        let resp = self
            .client
            .do_request(
                Method::POST,
                Some(bucket),
                None,
                Some(headers),
                Some(params),
                Some(body.into_bytes()),
            )
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await?;
            return Err(ObsError::service_error(status, &text));
        }

        Ok(DeleteObjectsOutput {
            deleted: self.inner.keys.clone(),
        })
    }
}

/// Input for the DeleteObjects operation.
#[derive(Debug, Clone, Default)]
pub struct DeleteObjectsInput {
    bucket: String,
    keys: Vec<DeleteObject>,
    quiet: bool,
}

/// Object to delete.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteObject {
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "VersionId", skip_serializing_if = "Option::is_none")]
    version_id: Option<String>,
}

/// Output for the DeleteObjects operation.
#[derive(Debug, Clone)]
pub struct DeleteObjectsOutput {
    deleted: Vec<DeleteObject>,
}

impl DeleteObjectsOutput {
    /// Get the deleted objects.
    pub fn deleted(&self) -> &[DeleteObject] {
        &self.deleted
    }
}

#[derive(Debug, Serialize)]
struct DeleteRequest {
    #[serde(rename = "Quiet")]
    quiet: bool,
    #[serde(rename = "Object")]
    object: Vec<DeleteObject>,
}

// ========================================
// Copy Object
// ========================================

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

        let copy_source = encode(&self.inner.copy_source);
        headers.insert(
            "x-obs-copy-source",
            HeaderValue::from_str(&format!("/{}", copy_source)).unwrap(),
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
            serde_xml_rs::from_str(&text).map_err(|e| ObsError::XmlParse(e.to_string()))?;

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

// ========================================
// Head Object
// ========================================

/// Fluent builder for the HeadObject operation.
#[derive(Debug, Clone)]
pub struct HeadObjectFluentBuilder {
    client: Client,
    inner: HeadObjectInput,
}

impl HeadObjectFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: HeadObjectInput::default(),
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
    pub async fn send(&self) -> Result<HeadObjectOutput> {
        let bucket = &self.inner.bucket;
        let key = &self.inner.key;

        if bucket.is_empty() {
            return Err(ObsError::InvalidInput("bucket name is required".to_string()));
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
            .do_request(Method::HEAD, Some(bucket), Some(key), None, Some(params), None)
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await.unwrap_or_default();
            return Err(ObsError::service_error(status, &text));
        }

        let headers = resp.headers();

        let content_type = headers
            .get("Content-Type")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let content_length = headers
            .get("Content-Length")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok());

        let etag = headers
            .get("ETag")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.trim_matches('"').to_string());

        let last_modified = headers
            .get("Last-Modified")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        let storage_class = headers
            .get("x-obs-storage-class")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.to_string());

        Ok(HeadObjectOutput {
            content_type,
            content_length,
            etag,
            last_modified,
            storage_class,
        })
    }
}

/// Input for the HeadObject operation.
#[derive(Debug, Clone, Default)]
pub struct HeadObjectInput {
    bucket: String,
    key: String,
    version_id: Option<String>,
}

/// Output for the HeadObject operation.
#[derive(Debug, Clone)]
pub struct HeadObjectOutput {
    content_type: Option<String>,
    content_length: Option<u64>,
    etag: Option<String>,
    last_modified: Option<String>,
    storage_class: Option<String>,
}

impl HeadObjectOutput {
    /// Get the content type.
    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }

    /// Get the content length.
    pub fn content_length(&self) -> Option<u64> {
        self.content_length
    }

    /// Get the ETag.
    pub fn etag(&self) -> Option<&str> {
        self.etag.as_deref()
    }

    /// Get the last modified time.
    pub fn last_modified(&self) -> Option<&str> {
        self.last_modified.as_deref()
    }

    /// Get the storage class.
    pub fn storage_class(&self) -> Option<&str> {
        self.storage_class.as_deref()
    }
}

// ========================================
// Append Object
// ========================================

/// Fluent builder for the AppendObject operation.
#[derive(Debug, Clone)]
pub struct AppendObjectFluentBuilder {
    client: Client,
    inner: AppendObjectInput,
}

impl AppendObjectFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: AppendObjectInput::default(),
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

    /// Set the position to append at.
    pub fn position(mut self, position: u64) -> Self {
        self.inner.position = position;
        self
    }

    /// Set the body to append.
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.inner.body = Some(body);
        self
    }

    /// Set the content type.
    pub fn content_type(mut self, content_type: impl Into<String>) -> Self {
        self.inner.content_type = Some(content_type.into());
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<AppendObjectOutput> {
        let bucket = &self.inner.bucket;
        let key = &self.inner.key;

        if bucket.is_empty() {
            return Err(ObsError::InvalidInput("bucket name is required".to_string()));
        }
        if key.is_empty() {
            return Err(ObsError::InvalidInput("object key is required".to_string()));
        }

        let body = self.inner.body.clone().unwrap_or_default();

        let mut params = HashMap::new();
        params.insert("append".to_string(), String::new());
        params.insert("position".to_string(), self.inner.position.to_string());

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

        let resp = self
            .client
            .do_request(Method::POST, Some(bucket), Some(key), Some(headers), Some(params), Some(body))
            .await?;

        let status = resp.status();
        let response_headers = resp.headers().clone();

        if !status.is_success() {
            let text = resp.text().await?;
            return Err(ObsError::service_error(status, &text));
        }

        let next_position = response_headers
            .get("x-obs-next-append-position")
            .and_then(|v| v.to_str().ok())
            .and_then(|s| s.parse::<u64>().ok());

        let etag = response_headers
            .get("ETag")
            .and_then(|v| v.to_str().ok())
            .map(|s| s.trim_matches('"').to_string());

        Ok(AppendObjectOutput {
            next_position,
            etag,
        })
    }
}

/// Input for the AppendObject operation.
#[derive(Debug, Clone, Default)]
pub struct AppendObjectInput {
    bucket: String,
    key: String,
    position: u64,
    body: Option<Vec<u8>>,
    content_type: Option<String>,
}

/// Output for the AppendObject operation.
#[derive(Debug, Clone)]
pub struct AppendObjectOutput {
    next_position: Option<u64>,
    etag: Option<String>,
}

impl AppendObjectOutput {
    /// Get the next append position.
    pub fn next_position(&self) -> Option<u64> {
        self.next_position
    }

    /// Get the ETag.
    pub fn etag(&self) -> Option<&str> {
        self.etag.as_deref()
    }
}