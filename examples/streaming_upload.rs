//! Example: Streaming upload an object to OBS
//!
//! This example demonstrates how to use streaming upload with the put_object operation.
//! Streaming upload is useful for:
//! - Large files that don't fit in memory
//! - Uploading data from a stream (e.g., from another HTTP request)
//! - Progress tracking during upload
//!
//! Usage:
//! 1. Create a .env file with:
//!    OBS_ACCESS_KEY_ID=your_access_key
//!    OBS_SECRET_ACCESS_KEY=your_secret_key
//!    OBS_BUCKET=your_bucket_name
//!    OBS_ENDPOINT=obs.cn-north-4.myhuaweicloud.com

use std::env;

use futures::stream;
use huaweicloud_sdk_rust_obs::{Client, Config, ObsError};
use reqwest::Body;

#[tokio::main]
async fn main() -> Result<(), ObsError> {
    // Load environment variables
    dotenvy::dotenv().ok();

    let access_key_id = env::var("OBS_ACCESS_KEY_ID").expect("OBS_ACCESS_KEY_ID must be set");
    let secret_access_key = env::var("OBS_SECRET_ACCESS_KEY").expect("OBS_SECRET_ACCESS_KEY must be set");
    let bucket = env::var("OBS_BUCKET").expect("OBS_BUCKET must be set");
    let endpoint = env::var("OBS_ENDPOINT").expect("OBS_ENDPOINT must be set");

    println!("Bucket: {}", bucket);

    // Create a client using AWS SDK style
    let config = Config::builder()
        .access_key(access_key_id, secret_access_key)
        .endpoint(&endpoint)
        .build()?;

    let client = Client::from_config(config)?;

    // ========================================
    // Example 1: Streaming upload from a byte stream
    // ========================================
    println!("\n[Example 1] Streaming upload from a byte stream...");
    
    // Create some data chunks
    let data1 = bytes::Bytes::from("Hello, ");
    let data2 = bytes::Bytes::from("OBS ");
    let data3 = bytes::Bytes::from("Streaming!");
    
    let total_size = data1.len() + data2.len() + data3.len();
    
    // Create a stream from the chunks
    let stream = stream::iter(vec![Ok::<_, std::io::Error>(data1), Ok(data2), Ok(data3)]);
    let body = Body::wrap_stream(stream);
    
    let key = "streaming-upload-test.txt";
    let result = client
        .put_object()
        .bucket(&bucket)
        .key(key)
        .streaming_body(body)
        .content_length(total_size as u64)
        .content_type("text/plain")
        .send()
        .await?;

    println!("Uploaded object: {}", key);
    if let Some(etag) = result.etag() {
        println!("ETag: {}", etag);
    }

    // ========================================
    // Example 2: Streaming upload from a file using async channel
    // ========================================
    println!("\n[Example 2] Streaming upload from a file...");
    
    // Read file content
    let file_path = "testdata/test.jpeg";
    let file_content = tokio::fs::read(file_path).await.expect("Failed to read file");
    let file_size = file_content.len() as u64;
    
    println!("File size: {} bytes", file_size);
    
    // Create a stream from the file content (split into chunks for demonstration)
    let chunk_size = 1024;
    let chunks: Vec<Result<bytes::Bytes, std::io::Error>> = file_content
        .chunks(chunk_size)
        .map(|chunk| Ok(bytes::Bytes::copy_from_slice(chunk)))
        .collect();
    
    let stream = stream::iter(chunks);
    let body = Body::wrap_stream(stream);
    
    let key = "streaming-upload-test.jpeg";
    let result = client
        .put_object()
        .bucket(&bucket)
        .key(key)
        .streaming_body(body)
        .content_length(file_size)
        .content_type("image/jpeg")
        .send()
        .await?;

    println!("Uploaded object: {}", key);
    if let Some(etag) = result.etag() {
        println!("ETag: {}", etag);
    }

    // ========================================
    // Example 3: Regular upload (for comparison)
    // ========================================
    println!("\n[Example 3] Regular upload (non-streaming)...");
    
    let key = "regular-upload-test.txt";
    let content = b"Hello, OBS! This is a regular upload.";
    let result = client
        .put_object()
        .bucket(&bucket)
        .key(key)
        .body(content.to_vec())
        .content_type("text/plain")
        .send()
        .await?;

    println!("Uploaded object: {}", key);
    if let Some(etag) = result.etag() {
        println!("ETag: {}", etag);
    }

    println!("\nAll uploads completed successfully!");
    Ok(())
}