//! Huawei Cloud OBS Rust SDK
//!
//! An unofficial Rust SDK for Huawei Cloud Object Storage Service (OBS).
//!
//! # Features
//!
//! - Fluent builder API
//! - Async/await support with Tokio
//! - Comprehensive error handling
//! - Support for bucket and object operations
//!
//! # Quick Start
//!
//! ```rust,no_run
//! use huaweicloud_sdk_rust_obs::{Client, Config, Region};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a client
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
//!     // Put an object
//!     let data = b"Hello, World!";
//!     client.put_object()
//!         .bucket("my-bucket")
//!         .key("hello.txt")
//!         .body(data.to_vec())
//!         .send()
//!         .await?;
//!
//!     // Get an object
//!     let result = client.get_object()
//!         .bucket("my-bucket")
//!         .key("hello.txt")
//!         .send()
//!         .await?;
//!
//!     println!("Content: {:?}", result.body());
//!
//!     Ok(())
//! }
//! ```
//!
//! # API Design
//!
//! This SDK uses fluent builder pattern for API design:
//!
//! - **Fluent Builders**: Each operation has a dedicated fluent builder
//! - **Input/Output Types**: Strongly typed request and response structures
//! - **Async by Default**: All operations are async using Tokio
//! - **Error Handling**: Comprehensive error types with detailed information
//!
//! # Operations
//!
//! ## Bucket Operations
//!
//! - [`Client::list_buckets`] - List all buckets
//! - [`Client::create_bucket`] - Create a new bucket
//! - [`Client::delete_bucket`] - Delete a bucket
//! - [`Client::get_bucket_location`] - Get bucket location
//! - [`Client::list_objects`] - List objects in a bucket (v1)
//! - [`Client::list_objects_v2`] - List objects in a bucket (v2)
//!
//! ## Object Operations
//!
//! - [`Client::put_object`] - Upload an object
//! - [`Client::get_object`] - Download an object
//! - [`Client::delete_object`] - Delete an object
//! - [`Client::delete_objects`] - Delete multiple objects
//! - [`Client::copy_object`] - Copy an object
//! - [`Client::head_object`] - Get object metadata
//! - [`Client::append_object`] - Append to an object

pub mod auth;
pub mod client;
pub mod config;
pub mod error;
pub mod operations;
mod xml_utils;

// Re-export main types for convenience
pub use client::{Client, ClientBuilder};
pub use config::{Config, ConfigBuilder, Credentials, Region, SignatureType};
pub use error::{ObsError, Result};

// Re-export operation types
pub use operations::{
    AbortMultipartUploadFluentBuilder,
    AbortMultipartUploadOutput,
    AppendObjectFluentBuilder,
    AppendObjectOutput,

    Bucket,
    CompleteMultipartUploadFluentBuilder,
    CompleteMultipartUploadOutput,
    CompletedPart,
    CopyObjectFluentBuilder,
    CopyObjectOutput,
    CopyPartFluentBuilder,
    CopyPartOutput,
    CreateBucketFluentBuilder,
    CreateBucketOutput,
    DeleteBucketFluentBuilder,
    DeleteBucketOutput,
    DeleteObjectFluentBuilder,
    DeleteObjectOutput,
    DeleteObjectsFluentBuilder,
    DeleteObjectsOutput,
    GetBucketLocationFluentBuilder,
    GetBucketLocationOutput,
    GetObjectFluentBuilder,
    GetObjectOutput,
    HeadObjectFluentBuilder,
    HeadObjectOutput,
    InitiateMultipartUploadFluentBuilder,
    InitiateMultipartUploadOutput,
    // Bucket operations
    ListBucketsFluentBuilder,
    ListBucketsOutput,
    // Multipart upload operations
    ListMultipartUploadsFluentBuilder,
    ListMultipartUploadsOutput,
    ListObjectsFluentBuilder,
    ListObjectsOutput,
    ListObjectsV2FluentBuilder,
    ListObjectsV2Output,
    ListPartsFluentBuilder,
    ListPartsOutput,
    MultipartInitiator,
    MultipartOwner,
    MultipartUpload,
    ObjectInfo,

    Owner,
    PartInfo,
    PutObjectBody,
    // Object operations
    PutObjectFluentBuilder,
    PutObjectOutput,
    UploadPartBody,
    UploadPartFluentBuilder,
    UploadPartOutput,
};

/// Prelude module for convenient imports.
pub mod prelude {
    pub use crate::client::Client;
    pub use crate::config::{Config, Credentials, Region};
    pub use crate::error::{ObsError, Result};
}
