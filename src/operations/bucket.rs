//! Bucket operations - AWS SDK style fluent builders.

use std::collections::HashMap;

use reqwest::Method;
use serde::{Deserialize, Serialize};

use crate::client::Client;
use crate::error::{ObsError, Result};

// ========================================
// List Buckets
// ========================================

/// Fluent builder for the ListBuckets operation.
#[derive(Debug, Clone)]
pub struct ListBucketsFluentBuilder {
    client: Client,
}

impl ListBucketsFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self { client }
    }

    /// Send the request.
    pub async fn send(&self) -> Result<ListBucketsOutput> {
        let resp = self
            .client
            .do_request(Method::GET, None, None, None, None, None)
            .await?;

        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            return Err(ObsError::service_error(status, &text));
        }

        let result: ListAllMyBucketsResult =
            serde_xml_rs::from_str(&text).map_err(|e| ObsError::XmlParse(e.to_string()))?;

        Ok(ListBucketsOutput::from(result))
    }
}

/// Output for the ListBuckets operation.
#[derive(Debug, Clone)]
pub struct ListBucketsOutput {
    owner: Owner,
    buckets: Vec<Bucket>,
}

impl ListBucketsOutput {
    /// Get the owner.
    pub fn owner(&self) -> &Owner {
        &self.owner
    }

    /// Get the list of buckets.
    pub fn buckets(&self) -> &[Bucket] {
        &self.buckets
    }

    /// Check if there are any buckets.
    pub fn is_empty(&self) -> bool {
        self.buckets.is_empty()
    }

    /// Get the number of buckets.
    pub fn len(&self) -> usize {
        self.buckets.len()
    }
}

impl From<ListAllMyBucketsResult> for ListBucketsOutput {
    fn from(value: ListAllMyBucketsResult) -> Self {
        Self {
            owner: value.owner,
            buckets: value.buckets.bucket,
        }
    }
}

/// Owner information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Owner {
    #[serde(rename = "ID")]
    id: String,
}

impl Owner {
    /// Get the owner ID.
    pub fn id(&self) -> &str {
        &self.id
    }
}

/// Bucket information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bucket {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "CreationDate")]
    creation_date: String,
    #[serde(rename = "Location")]
    location: String,
    #[serde(rename = "BucketType")]
    bucket_type: String,
}

impl Bucket {
    /// Get the bucket name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the creation date.
    pub fn creation_date(&self) -> &str {
        &self.creation_date
    }

    /// Get the location.
    pub fn location(&self) -> &str {
        &self.location
    }

    /// Get the bucket type.
    pub fn bucket_type(&self) -> &str {
        &self.bucket_type
    }
}

// Internal XML response type
#[derive(Debug, Serialize, Deserialize)]
struct ListAllMyBucketsResult {
    #[serde(rename = "Owner")]
    owner: Owner,
    #[serde(rename = "Buckets")]
    buckets: Buckets,
}

#[derive(Debug, Serialize, Deserialize)]
struct Buckets {
    #[serde(rename = "Bucket")]
    bucket: Vec<Bucket>,
}

// ========================================
// Create Bucket
// ========================================

/// Fluent builder for the CreateBucket operation.
#[derive(Debug, Clone)]
pub struct CreateBucketFluentBuilder {
    client: Client,
    inner: CreateBucketInput,
}

impl CreateBucketFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: CreateBucketInput::default(),
        }
    }

    /// Set the bucket name.
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.inner.bucket = bucket.into();
        self
    }

    /// Set the location constraint.
    pub fn location_constraint(mut self, location: impl Into<String>) -> Self {
        self.inner.location_constraint = Some(location.into());
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<CreateBucketOutput> {
        let bucket = &self.inner.bucket;
        if bucket.is_empty() {
            return Err(ObsError::InvalidInput("bucket name is required".to_string()));
        }

        let default_location = self.client.config().region().name().to_string();
        let location = self
            .inner
            .location_constraint
            .as_deref()
            .unwrap_or(&default_location);

        let xml = CreateBucketConfiguration {
            location: location.to_string(),
        };
        let body = serde_xml_rs::to_string(&xml)
            .map_err(|e| ObsError::Serialization(e.to_string()))?;

        let resp = self
            .client
            .do_request(Method::PUT, Some(bucket), None, None, None, Some(body.into_bytes()))
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await?;
            return Err(ObsError::service_error(status, &text));
        }

        Ok(CreateBucketOutput {
            location: bucket.to_string(),
        })
    }
}

/// Input for the CreateBucket operation.
#[derive(Debug, Clone, Default)]
pub struct CreateBucketInput {
    bucket: String,
    location_constraint: Option<String>,
}

/// Output for the CreateBucket operation.
#[derive(Debug, Clone)]
pub struct CreateBucketOutput {
    location: String,
}

impl CreateBucketOutput {
    /// Get the bucket name.
    pub fn location(&self) -> &str {
        &self.location
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct CreateBucketConfiguration {
    #[serde(rename = "Location")]
    location: String,
}

// ========================================
// Delete Bucket
// ========================================

/// Fluent builder for the DeleteBucket operation.
#[derive(Debug, Clone)]
pub struct DeleteBucketFluentBuilder {
    client: Client,
    inner: DeleteBucketInput,
}

impl DeleteBucketFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: DeleteBucketInput::default(),
        }
    }

    /// Set the bucket name.
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.inner.bucket = bucket.into();
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<DeleteBucketOutput> {
        let bucket = &self.inner.bucket;
        if bucket.is_empty() {
            return Err(ObsError::InvalidInput("bucket name is required".to_string()));
        }

        let resp = self
            .client
            .do_request(Method::DELETE, Some(bucket), None, None, None, None)
            .await?;

        let status = resp.status();
        if !status.is_success() {
            let text = resp.text().await?;
            return Err(ObsError::service_error(status, &text));
        }

        Ok(DeleteBucketOutput {})
    }
}

/// Input for the DeleteBucket operation.
#[derive(Debug, Clone, Default)]
pub struct DeleteBucketInput {
    bucket: String,
}

/// Output for the DeleteBucket operation.
#[derive(Debug, Clone)]
pub struct DeleteBucketOutput {}

// ========================================
// Get Bucket Location
// ========================================

/// Fluent builder for the GetBucketLocation operation.
#[derive(Debug, Clone)]
pub struct GetBucketLocationFluentBuilder {
    client: Client,
    inner: GetBucketLocationInput,
}

impl GetBucketLocationFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: GetBucketLocationInput::default(),
        }
    }

    /// Set the bucket name.
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.inner.bucket = bucket.into();
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<GetBucketLocationOutput> {
        let bucket = &self.inner.bucket;
        if bucket.is_empty() {
            return Err(ObsError::InvalidInput("bucket name is required".to_string()));
        }

        let mut params = HashMap::new();
        params.insert("location".to_string(), String::new());

        let resp = self
            .client
            .do_request(Method::GET, Some(bucket), None, None, Some(params), None)
            .await?;

        let status = resp.status();
        let text = resp.text().await?;

        if !status.is_success() {
            return Err(ObsError::service_error(status, &text));
        }

        let result: Location =
            serde_xml_rs::from_str(&text).map_err(|e| ObsError::XmlParse(e.to_string()))?;

        Ok(GetBucketLocationOutput {
            location: result.location,
        })
    }
}

/// Input for the GetBucketLocation operation.
#[derive(Debug, Clone, Default)]
pub struct GetBucketLocationInput {
    bucket: String,
}

/// Output for the GetBucketLocation operation.
#[derive(Debug, Clone)]
pub struct GetBucketLocationOutput {
    location: String,
}

impl GetBucketLocationOutput {
    /// Get the location.
    pub fn location(&self) -> &str {
        &self.location
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Location {
    #[serde(rename = "Location")]
    location: String,
}

// ========================================
// List Objects
// ========================================

/// Fluent builder for the ListObjects operation.
#[derive(Debug, Clone)]
pub struct ListObjectsFluentBuilder {
    client: Client,
    inner: ListObjectsInput,
}

impl ListObjectsFluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: ListObjectsInput::default(),
        }
    }

    /// Set the bucket name.
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.inner.bucket = bucket.into();
        self
    }

    /// Set the delimiter.
    pub fn delimiter(mut self, delimiter: impl Into<String>) -> Self {
        self.inner.delimiter = Some(delimiter.into());
        self
    }

    /// Set the encoding type.
    pub fn encoding_type(mut self, encoding_type: impl Into<String>) -> Self {
        self.inner.encoding_type = Some(encoding_type.into());
        self
    }

    /// Set the marker.
    pub fn marker(mut self, marker: impl Into<String>) -> Self {
        self.inner.marker = Some(marker.into());
        self
    }

    /// Set the max keys.
    pub fn max_keys(mut self, max_keys: i32) -> Self {
        self.inner.max_keys = Some(max_keys);
        self
    }

    /// Set the prefix.
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.inner.prefix = Some(prefix.into());
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<ListObjectsOutput> {
        let bucket = &self.inner.bucket;
        if bucket.is_empty() {
            return Err(ObsError::InvalidInput("bucket name is required".to_string()));
        }

        let mut params = HashMap::new();
        params.insert("delimiter".to_string(), self.inner.delimiter.as_deref().unwrap_or("/").to_string());

        if let Some(ref marker) = self.inner.marker {
            params.insert("marker".to_string(), marker.clone());
        }

        if let Some(max_keys) = self.inner.max_keys {
            params.insert("max-keys".to_string(), max_keys.to_string());
        }

        if let Some(ref prefix) = self.inner.prefix {
            params.insert("prefix".to_string(), prefix.clone());
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

        let result: ListBucketResult =
            serde_xml_rs::from_str(&text).map_err(|e| ObsError::XmlParse(e.to_string()))?;

        Ok(ListObjectsOutput::from(result))
    }
}

/// Input for the ListObjects operation.
#[derive(Debug, Clone, Default)]
pub struct ListObjectsInput {
    bucket: String,
    delimiter: Option<String>,
    encoding_type: Option<String>,
    marker: Option<String>,
    max_keys: Option<i32>,
    prefix: Option<String>,
}

/// Output for the ListObjects operation.
#[derive(Debug, Clone)]
pub struct ListObjectsOutput {
    name: String,
    prefix: Option<String>,
    delimiter: Option<String>,
    marker: Option<String>,
    max_keys: i32,
    is_truncated: bool,
    next_marker: Option<String>,
    contents: Vec<ObjectInfo>,
    common_prefixes: Vec<String>,
}

impl ListObjectsOutput {
    /// Get the bucket name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the prefix.
    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
    }

    /// Get the delimiter.
    pub fn delimiter(&self) -> Option<&str> {
        self.delimiter.as_deref()
    }

    /// Get the marker.
    pub fn marker(&self) -> Option<&str> {
        self.marker.as_deref()
    }

    /// Get the max keys.
    pub fn max_keys(&self) -> i32 {
        self.max_keys
    }

    /// Check if the result is truncated.
    pub fn is_truncated(&self) -> bool {
        self.is_truncated
    }

    /// Get the next marker.
    pub fn next_marker(&self) -> Option<&str> {
        self.next_marker.as_deref()
    }

    /// Get the contents.
    pub fn contents(&self) -> &[ObjectInfo] {
        &self.contents
    }

    /// Get the common prefixes.
    pub fn common_prefixes(&self) -> &[String] {
        &self.common_prefixes
    }
}

impl From<ListBucketResult> for ListObjectsOutput {
    fn from(value: ListBucketResult) -> Self {
        Self {
            name: value.name,
            prefix: value.prefix,
            delimiter: value.delimiter,
            marker: None,
            max_keys: value.max_keys.parse().unwrap_or(1000),
            is_truncated: value.is_truncated == "true",
            next_marker: value.next_marker,
            contents: value.contents.unwrap_or_default(),
            common_prefixes: value
                .common_prefixes
                .unwrap_or_default()
                .into_iter()
                .map(|p| p.prefix)
                .collect(),
        }
    }
}

/// Object information.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectInfo {
    #[serde(rename = "Key")]
    key: String,
    #[serde(rename = "LastModified")]
    last_modified: String,
    #[serde(rename = "ETag")]
    etag: String,
    #[serde(rename = "Size")]
    size: i64,
    #[serde(rename = "StorageClass")]
    storage_class: String,
    #[serde(rename = "Owner")]
    owner: Option<ObjectOwner>,
}

impl ObjectInfo {
    /// Get the key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Get the last modified time.
    pub fn last_modified(&self) -> &str {
        &self.last_modified
    }

    /// Get the ETag.
    pub fn etag(&self) -> &str {
        &self.etag
    }

    /// Get the size.
    pub fn size(&self) -> i64 {
        self.size
    }

    /// Get the storage class.
    pub fn storage_class(&self) -> &str {
        &self.storage_class
    }

    /// Get the owner.
    pub fn owner(&self) -> Option<&ObjectOwner> {
        self.owner.as_ref()
    }
}

/// Object owner.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectOwner {
    #[serde(rename = "ID")]
    id: String,
}

impl ObjectOwner {
    /// Get the owner ID.
    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ListBucketResult {
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Prefix")]
    prefix: Option<String>,
    #[serde(rename = "Delimiter")]
    delimiter: Option<String>,
    #[serde(rename = "EncodingType")]
    encoding_type: Option<String>,
    #[serde(rename = "NextMarker")]
    next_marker: Option<String>,
    #[serde(rename = "MaxKeys")]
    max_keys: String,
    #[serde(rename = "IsTruncated")]
    is_truncated: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Contents")]
    contents: Option<Vec<ObjectInfo>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "CommonPrefixes")]
    common_prefixes: Option<Vec<CommonPrefix>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommonPrefix {
    #[serde(rename = "Prefix")]
    prefix: String,
}

// ========================================
// List Objects V2
// ========================================

/// Fluent builder for the ListObjectsV2 operation.
#[derive(Debug, Clone)]
pub struct ListObjectsV2FluentBuilder {
    client: Client,
    inner: ListObjectsV2Input,
}

impl ListObjectsV2FluentBuilder {
    /// Create a new fluent builder.
    pub(crate) fn new(client: Client) -> Self {
        Self {
            client,
            inner: ListObjectsV2Input::default(),
        }
    }

    /// Set the bucket name.
    pub fn bucket(mut self, bucket: impl Into<String>) -> Self {
        self.inner.bucket = bucket.into();
        self
    }

    /// Set the continuation token.
    pub fn continuation_token(mut self, token: impl Into<String>) -> Self {
        self.inner.continuation_token = Some(token.into());
        self
    }

    /// Set the delimiter.
    pub fn delimiter(mut self, delimiter: impl Into<String>) -> Self {
        self.inner.delimiter = Some(delimiter.into());
        self
    }

    /// Set the max keys.
    pub fn max_keys(mut self, max_keys: i32) -> Self {
        self.inner.max_keys = Some(max_keys);
        self
    }

    /// Set the prefix.
    pub fn prefix(mut self, prefix: impl Into<String>) -> Self {
        self.inner.prefix = Some(prefix.into());
        self
    }

    /// Set the start after.
    pub fn start_after(mut self, start_after: impl Into<String>) -> Self {
        self.inner.start_after = Some(start_after.into());
        self
    }

    /// Send the request.
    pub async fn send(&self) -> Result<ListObjectsV2Output> {
        let bucket = &self.inner.bucket;
        if bucket.is_empty() {
            return Err(ObsError::InvalidInput("bucket name is required".to_string()));
        }

        let mut params = HashMap::new();
        params.insert("list-type".to_string(), "2".to_string());

        if let Some(ref delimiter) = self.inner.delimiter {
            params.insert("delimiter".to_string(), delimiter.clone());
        }

        if let Some(ref token) = self.inner.continuation_token {
            params.insert("continuation-token".to_string(), token.clone());
        }

        if let Some(max_keys) = self.inner.max_keys {
            params.insert("max-keys".to_string(), max_keys.to_string());
        }

        if let Some(ref prefix) = self.inner.prefix {
            params.insert("prefix".to_string(), prefix.clone());
        }

        if let Some(ref start_after) = self.inner.start_after {
            params.insert("start-after".to_string(), start_after.clone());
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

        // Parse V2 response (similar to V1 but with different fields)
        let result: ListBucketResult =
            serde_xml_rs::from_str(&text).map_err(|e| ObsError::XmlParse(e.to_string()))?;

        Ok(ListObjectsV2Output::from(result))
    }
}

/// Input for the ListObjectsV2 operation.
#[derive(Debug, Clone, Default)]
pub struct ListObjectsV2Input {
    bucket: String,
    continuation_token: Option<String>,
    delimiter: Option<String>,
    max_keys: Option<i32>,
    prefix: Option<String>,
    start_after: Option<String>,
}

/// Output for the ListObjectsV2 operation.
#[derive(Debug, Clone)]
pub struct ListObjectsV2Output {
    name: String,
    prefix: Option<String>,
    delimiter: Option<String>,
    max_keys: i32,
    is_truncated: bool,
    next_continuation_token: Option<String>,
    contents: Vec<ObjectInfo>,
    common_prefixes: Vec<String>,
}

impl ListObjectsV2Output {
    /// Get the bucket name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the prefix.
    pub fn prefix(&self) -> Option<&str> {
        self.prefix.as_deref()
    }

    /// Get the delimiter.
    pub fn delimiter(&self) -> Option<&str> {
        self.delimiter.as_deref()
    }

    /// Get the max keys.
    pub fn max_keys(&self) -> i32 {
        self.max_keys
    }

    /// Check if the result is truncated.
    pub fn is_truncated(&self) -> bool {
        self.is_truncated
    }

    /// Get the next continuation token.
    pub fn next_continuation_token(&self) -> Option<&str> {
        self.next_continuation_token.as_deref()
    }

    /// Get the contents.
    pub fn contents(&self) -> &[ObjectInfo] {
        &self.contents
    }

    /// Get the common prefixes.
    pub fn common_prefixes(&self) -> &[String] {
        &self.common_prefixes
    }
}

impl From<ListBucketResult> for ListObjectsV2Output {
    fn from(value: ListBucketResult) -> Self {
        Self {
            name: value.name,
            prefix: value.prefix,
            delimiter: value.delimiter,
            max_keys: value.max_keys.parse().unwrap_or(1000),
            is_truncated: value.is_truncated == "true",
            next_continuation_token: value.next_marker,
            contents: value.contents.unwrap_or_default(),
            common_prefixes: value
                .common_prefixes
                .unwrap_or_default()
                .into_iter()
                .map(|p| p.prefix)
                .collect(),
        }
    }
}