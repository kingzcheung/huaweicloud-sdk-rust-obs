//! OBS Client - The main entry point for interacting with Huawei Cloud OBS.
//!
//! This module provides the main client for OBS operations, following AWS SDK style.
//!
//! # Example
//!
//! ```rust,no_run
//! use huaweicloud_sdk_rust_obs::{Client, Config, Region};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = Config::builder()
//!         .access_key("your-access-key", "your-secret-key")
//!         .region_name("cn-north-4")
//!         .build()?;
//!
//!     let client = Client::from_config(config);
//!
//!     // List buckets
//!     let result = client.list_buckets().send().await?;
//!     for bucket in result.buckets() {
//!         println!("Bucket: {}", bucket.name());
//!     }
//!
//!     Ok(())
//! }
//! ```

use std::collections::HashMap;
use std::time::Duration;

use reqwest::{header::HeaderMap, Body, Method, Response};

use crate::auth::Authorization;
use crate::config::{Config, SignatureType};
use crate::error::{ObsError, Result};
use crate::operations::*;

/// The main client for OBS operations.
///
/// This client is thread-safe and can be cloned cheaply.
/// It maintains a connection pool internally for efficient HTTP requests.
#[derive(Debug, Clone)]
pub struct Client {
    config: Config,
    http_client: reqwest::Client,
}

impl Client {
    /// Create a new client from a configuration.
    pub fn from_config(config: Config) -> Result<Self> {
        let http_client = reqwest::ClientBuilder::new()
            .timeout(config.timeout())
            .connect_timeout(config.connect_timeout())
            .pool_max_idle_per_host(10)
            .pool_idle_timeout(Duration::from_secs(60))
            .build()
            .map_err(|e| ObsError::ClientBuild(e.to_string()))?;

        Ok(Self {
            config,
            http_client,
        })
    }

    /// Create a new client builder.
    pub fn builder() -> ClientBuilder {
        ClientBuilder::new()
    }

    /// Get the configuration.
    pub fn config(&self) -> &Config {
        &self.config
    }

    // ========================================
    // Bucket Operations
    // ========================================

    /// List all buckets.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let result = client.list_buckets().send().await?;
    /// for bucket in result.buckets() {
    ///     println!("Bucket: {}", bucket.name());
    /// }
    /// ```
    pub fn list_buckets(&self) -> ListBucketsFluentBuilder {
        ListBucketsFluentBuilder::new(self.clone())
    }

    /// Create a new bucket.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// client.create_bucket()
    ///     .bucket("my-bucket")
    ///     .send()
    ///     .await?;
    /// ```
    pub fn create_bucket(&self) -> CreateBucketFluentBuilder {
        CreateBucketFluentBuilder::new(self.clone())
    }

    /// Delete a bucket.
    pub fn delete_bucket(&self) -> DeleteBucketFluentBuilder {
        DeleteBucketFluentBuilder::new(self.clone())
    }

    /// Get bucket location.
    pub fn get_bucket_location(&self) -> GetBucketLocationFluentBuilder {
        GetBucketLocationFluentBuilder::new(self.clone())
    }

    /// List objects in a bucket.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let result = client.list_objects()
    ///     .bucket("my-bucket")
    ///     .prefix("photos/")
    ///     .max_keys(100)
    ///     .send()
    ///     .await?;
    ///
    /// for object in result.contents() {
    ///     println!("Object: {}", object.key());
    /// }
    /// ```
    pub fn list_objects(&self) -> ListObjectsFluentBuilder {
        ListObjectsFluentBuilder::new(self.clone())
    }

    /// List objects version 2 (recommended).
    pub fn list_objects_v2(&self) -> ListObjectsV2FluentBuilder {
        ListObjectsV2FluentBuilder::new(self.clone())
    }

    // ========================================
    // Object Operations
    // ========================================

    /// Put an object into a bucket.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let data = b"Hello, World!";
    /// let result = client.put_object()
    ///     .bucket("my-bucket")
    ///     .key("hello.txt")
    ///     .body(data.to_vec())
    ///     .send()
    ///     .await?;
    ///
    /// println!("ETag: {}", result.etag());
    /// ```
    pub fn put_object(&self) -> PutObjectFluentBuilder {
        PutObjectFluentBuilder::new(self.clone())
    }

    /// Get an object from a bucket.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let result = client.get_object()
    ///     .bucket("my-bucket")
    ///     .key("hello.txt")
    ///     .send()
    ///     .await?;
    ///
    /// let data = result.body().bytes().await?;
    /// println!("Content: {:?}", data);
    /// ```
    pub fn get_object(&self) -> GetObjectFluentBuilder {
        GetObjectFluentBuilder::new(self.clone())
    }

    /// Delete an object from a bucket.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// client.delete_object()
    ///     .bucket("my-bucket")
    ///     .key("hello.txt")
    ///     .send()
    ///     .await?;
    /// ```
    pub fn delete_object(&self) -> DeleteObjectFluentBuilder {
        DeleteObjectFluentBuilder::new(self.clone())
    }

    /// Delete multiple objects from a bucket.
    pub fn delete_objects(&self) -> DeleteObjectsFluentBuilder {
        DeleteObjectsFluentBuilder::new(self.clone())
    }

    /// Copy an object.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let result = client.copy_object()
    ///     .bucket("dest-bucket")
    ///     .key("dest-key")
    ///     .copy_source("source-bucket/source-key")
    ///     .send()
    ///     .await?;
    /// ```
    pub fn copy_object(&self) -> CopyObjectFluentBuilder {
        CopyObjectFluentBuilder::new(self.clone())
    }

    /// Get object metadata (HEAD request).
    pub fn head_object(&self) -> HeadObjectFluentBuilder {
        HeadObjectFluentBuilder::new(self.clone())
    }

    /// Append to an object.
    pub fn append_object(&self) -> AppendObjectFluentBuilder {
        AppendObjectFluentBuilder::new(self.clone())
    }

    /// Set object ACL.
    pub fn set_object_acl(&self) -> SetObjectAclFluentBuilder {
        SetObjectAclFluentBuilder::new(self.clone())
    }

    /// Get object ACL.
    pub fn get_object_acl(&self) -> GetObjectAclFluentBuilder {
        GetObjectAclFluentBuilder::new(self.clone())
    }

    // ========================================
    // Multipart Upload Operations
    // ========================================

    /// List in-progress multipart uploads.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let result = client.list_multipart_uploads()
    ///     .bucket("my-bucket")
    ///     .max_uploads(100)
    ///     .send()
    ///     .await?;
    ///
    /// for upload in result.uploads() {
    ///     println!("Key: {}, UploadId: {}", upload.key(), upload.upload_id());
    /// }
    /// ```
    pub fn list_multipart_uploads(&self) -> ListMultipartUploadsFluentBuilder {
        ListMultipartUploadsFluentBuilder::new(self.clone())
    }

    /// Initiate a multipart upload.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let result = client.initiate_multipart_upload()
    ///     .bucket("my-bucket")
    ///     .key("large-file.zip")
    ///     .send()
    ///     .await?;
    ///
    /// println!("Upload ID: {}", result.upload_id());
    /// ```
    pub fn initiate_multipart_upload(&self) -> InitiateMultipartUploadFluentBuilder {
        InitiateMultipartUploadFluentBuilder::new(self.clone())
    }

    /// Upload a part in a multipart upload.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let data = vec![0u8; 1024 * 1024]; // 1MB of data
    /// let result = client.upload_part()
    ///     .bucket("my-bucket")
    ///     .key("large-file.zip")
    ///     .upload_id("upload-id")
    ///     .part_number(1)
    ///     .body(data)
    ///     .send()
    ///     .await?;
    ///
    /// println!("ETag: {}", result.etag());
    /// ```
    pub fn upload_part(&self) -> UploadPartFluentBuilder {
        UploadPartFluentBuilder::new(self.clone())
    }

    /// Copy a part from an existing object to a multipart upload.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let result = client.copy_part()
    ///     .bucket("dest-bucket")
    ///     .key("dest-object")
    ///     .upload_id("upload-id")
    ///     .part_number(1)
    ///     .copy_source("/source-bucket/source-object")
    ///     .send()
    ///     .await?;
    ///
    /// println!("ETag: {}", result.etag());
    /// ```
    pub fn copy_part(&self) -> CopyPartFluentBuilder {
        CopyPartFluentBuilder::new(self.clone())
    }

    /// List parts that have been uploaded for a multipart upload.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let result = client.list_parts()
    ///     .bucket("my-bucket")
    ///     .key("large-file.zip")
    ///     .upload_id("upload-id")
    ///     .send()
    ///     .await?;
    ///
    /// for part in result.parts() {
    ///     println!("Part {}: ETag={}", part.part_number(), part.etag());
    /// }
    /// ```
    pub fn list_parts(&self) -> ListPartsFluentBuilder {
        ListPartsFluentBuilder::new(self.clone())
    }

    /// Complete a multipart upload by assembling previously uploaded parts.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// let result = client.complete_multipart_upload()
    ///     .bucket("my-bucket")
    ///     .key("large-file.zip")
    ///     .upload_id("upload-id")
    ///     .part(1, "etag1")
    ///     .part(2, "etag2")
    ///     .send()
    ///     .await?;
    ///
    /// println!("ETag: {}", result.etag());
    /// ```
    pub fn complete_multipart_upload(&self) -> CompleteMultipartUploadFluentBuilder {
        CompleteMultipartUploadFluentBuilder::new(self.clone())
    }

    /// Abort a multipart upload.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// client.abort_multipart_upload()
    ///     .bucket("my-bucket")
    ///     .key("large-file.zip")
    ///     .upload_id("upload-id")
    ///     .send()
    ///     .await?;
    /// ```
    pub fn abort_multipart_upload(&self) -> AbortMultipartUploadFluentBuilder {
        AbortMultipartUploadFluentBuilder::new(self.clone())
    }

    // ========================================
    // Internal Methods
    // ========================================

    /// Execute an HTTP request with authentication.
    pub(crate) async fn do_request(
        &self,
        method: Method,
        bucket: Option<&str>,
        key: Option<&str>,
        headers: Option<HeaderMap>,
        params: Option<HashMap<String, String>>,
        body: Option<Vec<u8>>,
    ) -> Result<Response> {
        let mut auth_headers: HashMap<String, Vec<String>> = HashMap::new();
        let mut req_headers = if let Some(h) = headers {
            for (k, v) in &h {
                if let Ok(v) = v.to_str() {
                    auth_headers.insert(k.as_str().to_string(), vec![v.to_string()]);
                }
            }
            h
        } else {
            HeaderMap::new()
        };

        let (request_uri, canonicalized_url) = self.format_urls(bucket, key, params.as_ref());

        let url = if let Some(bucket) = bucket {
            if request_uri.is_empty() {
                format!(
                    "https://{}.{}",
                    bucket,
                    self.config.region().endpoint()
                )
            } else {
                format!(
                    "https://{}.{}/{}",
                    bucket,
                    self.config.region().endpoint(),
                    &request_uri
                )
            }
        } else {
            // For ListBuckets (no bucket), URL should end with /
            format!("https://{}/", self.config.region().endpoint())
        };

        let auth_headers = self.auth(
            method.as_str(),
            bucket.unwrap_or(""),
            HashMap::new(),
            auth_headers,
            canonicalized_url,
        )?;
        req_headers.extend(auth_headers);

        let mut req = self.http_client.request(method, url).headers(req_headers);

        if let Some(body) = body {
            req = req.body(body);
        }

        let res = req.send().await?;
        Ok(res)
    }

    /// Execute an HTTP request with streaming body and authentication.
    pub(crate) async fn do_request_streaming(
        &self,
        method: Method,
        bucket: Option<&str>,
        key: Option<&str>,
        headers: Option<HeaderMap>,
        params: Option<HashMap<String, String>>,
        body: Option<Body>,
    ) -> Result<Response> {
        let mut auth_headers: HashMap<String, Vec<String>> = HashMap::new();
        let mut req_headers = if let Some(h) = headers {
            for (k, v) in &h {
                if let Ok(v) = v.to_str() {
                    auth_headers.insert(k.as_str().to_string(), vec![v.to_string()]);
                }
            }
            h
        } else {
            HeaderMap::new()
        };

        let (request_uri, canonicalized_url) = self.format_urls(bucket, key, params.as_ref());

        let url = if let Some(bucket) = bucket {
            if request_uri.is_empty() {
                format!(
                    "https://{}.{}",
                    bucket,
                    self.config.region().endpoint()
                )
            } else {
                format!(
                    "https://{}.{}/{}",
                    bucket,
                    self.config.region().endpoint(),
                    &request_uri
                )
            }
        } else {
            // For ListBuckets (no bucket), URL should end with /
            format!("https://{}/", self.config.region().endpoint())
        };

        let auth_headers = self.auth(
            method.as_str(),
            bucket.unwrap_or(""),
            HashMap::new(),
            auth_headers,
            canonicalized_url,
        )?;
        req_headers.extend(auth_headers);

        let mut req = self.http_client.request(method, url).headers(req_headers);

        if let Some(body) = body {
            req = req.body(body);
        }

        let res = req.send().await?;
        Ok(res)
    }

    /// Format URLs for the request.
    fn format_urls(
        &self,
        bucket_name: Option<&str>,
        object_key: Option<&str>,
        params: Option<&HashMap<String, String>>,
    ) -> (String, String) {
        let mut canonicalized_resource: String = String::from("/");
        let mut uri: String = String::new();

        if let Some(bucket) = bucket_name {
            canonicalized_resource.push_str(bucket);
            canonicalized_resource.push('/');

            match self.config.signature_type() {
                SignatureType::V2 | SignatureType::Obs => {
                    if let Some(key) = object_key {
                        if !key.is_empty() {
                            canonicalized_resource.push_str(key);
                            uri.push_str(key);
                        }
                    }
                    if let Some(params) = params {
                        // Sort params for consistent signature calculation
                        let mut sorted_params: Vec<_> = params.iter().collect();
                        sorted_params.sort_by(|a, b| a.0.cmp(b.0));
                        
                        canonicalized_resource.push('?');
                        let mut uri_params = vec![];
                        for (k, v) in &sorted_params {
                            if v.is_empty() {
                                uri_params.push(k.to_string());
                            } else {
                                uri_params.push(format!("{}={}", k, v));
                            }
                            if crate::config::SUB_RESOURCES.contains(&k.as_str()) {
                                if !canonicalized_resource.ends_with('?') {
                                    canonicalized_resource.push('&');
                                }
                                canonicalized_resource.push_str(k);
                                if !v.is_empty() {
                                    canonicalized_resource.push('=');
                                    canonicalized_resource.push_str(v);
                                }
                            }
                        }
                        if !uri_params.is_empty() {
                            uri.push('?');
                            uri.push_str(uri_params.join("&").as_str());
                        }
                    }
                }
                SignatureType::V4 => {
                    canonicalized_resource.push('/');
                }
            }
        }

        (uri, canonicalized_resource.trim_end_matches('?').to_string())
    }
}

/// Builder for creating a Client.
#[derive(Debug)]
pub struct ClientBuilder {
    config: Config,
}

impl ClientBuilder {
    fn new() -> Self {
        Self {
            config: Config::builder().build().unwrap_or_else(|_| {
                panic!("Failed to create default config. This should not happen.")
            }),
        }
    }

    /// Set the access key credentials.
    pub fn access_key(mut self, access_key_id: impl Into<String>, secret_access_key: impl Into<String>) -> Self {
        self.config = Config::builder()
            .access_key(access_key_id, secret_access_key)
            .region(self.config.region().clone())
            .build()
            .expect("Failed to build config");
        self
    }

    /// Set the endpoint.
    pub fn endpoint(mut self, endpoint: impl Into<String>) -> Self {
        let endpoint = endpoint.into();
        let endpoint = endpoint
            .replace("https://", "")
            .replace("http://", "");
        
        self.config = Config::builder()
            .access_key(
                self.config.credentials().access_key_id(),
                self.config.credentials().secret_access_key(),
            )
            .endpoint(endpoint)
            .build()
            .expect("Failed to build config");
        self
    }

    /// Set the region.
    pub fn region(mut self, region: impl Into<String>) -> Self {
        self.config = Config::builder()
            .access_key(
                self.config.credentials().access_key_id(),
                self.config.credentials().secret_access_key(),
            )
            .region_name(region)
            .build()
            .expect("Failed to build config");
        self
    }

    /// Set the request timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config = Config::builder()
            .access_key(
                self.config.credentials().access_key_id(),
                self.config.credentials().secret_access_key(),
            )
            .region(self.config.region().clone())
            .timeout(timeout)
            .build()
            .expect("Failed to build config");
        self
    }

    /// Build the client.
    pub fn build(self) -> Result<Client> {
        Client::from_config(self.config)
    }
}

impl std::fmt::Display for Client {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "OBS Client (region: {}, endpoint: {})",
            self.config.region().name(),
            self.config.region().endpoint()
        )
    }
}
