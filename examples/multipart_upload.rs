//! Example: Multipart upload for large files
//!
//! This example demonstrates how to use multipart upload for large files.
//! Multipart upload is useful for:
//! - Files larger than 5GB (required)
//! - Files larger than 100MB (recommended)
//! - Uploading over unstable networks (can retry individual parts)
//! - Parallel uploads for better performance
//!
//! Usage:
//! 1. Create a .env file with:
//!    OBS_ACCESS_KEY_ID=your_access_key
//!    OBS_SECRET_ACCESS_KEY=your_secret_key
//!    OBS_BUCKET=your_bucket_name
//!    OBS_ENDPOINT=obs.cn-north-4.myhuaweicloud.com

use std::env;

use huaweicloud_sdk_rust_obs::{Client, CompletedPart, Config, ObsError};

#[tokio::main]
async fn main() -> Result<(), ObsError> {
    // Load environment variables
    dotenvy::dotenv().ok();

    let access_key_id = env::var("OBS_ACCESS_KEY_ID").expect("OBS_ACCESS_KEY_ID must be set");
    let secret_access_key =
        env::var("OBS_SECRET_ACCESS_KEY").expect("OBS_SECRET_ACCESS_KEY must be set");
    let bucket = env::var("OBS_BUCKET").expect("OBS_BUCKET must be set");
    let endpoint = env::var("OBS_ENDPOINT").expect("OBS_ENDPOINT must be set");

    println!("Bucket: {}", bucket);

    // Create a client
    let config = Config::builder()
        .access_key(access_key_id, secret_access_key)
        .endpoint(&endpoint)
        .build()?;

    let client = Client::from_config(config)?;

    // ========================================
    // Example 1: Basic multipart upload
    // ========================================
    println!("\n[Example 1] Basic multipart upload...");

    let object_key = "multipart-upload-test.bin";
    let part_size = 100 * 1024; // 100KB per part (for demo, use 5MB+ in production)

    // Step 1: Initialize multipart upload
    println!("  Step 1: Initializing multipart upload...");
    let initiate_result = client
        .initiate_multipart_upload()
        .bucket(&bucket)
        .key(object_key)
        .content_type("application/octet-stream")
        .send()
        .await?;

    let upload_id = initiate_result.upload_id();
    println!("  Upload ID: {}", upload_id);

    // Step 2: Upload parts
    println!("  Step 2: Uploading parts...");
    let mut completed_parts: Vec<CompletedPart> = Vec::new();

    // Create test data for 3 parts
    let parts_data: Vec<Vec<u8>> = (0..3)
        .map(|part_num| {
            let size = if part_num == 2 { 50 * 1024 } else { part_size }; // Last part is smaller
            (0..size).map(|i| ((i + part_num * 100) % 256) as u8).collect()
        })
        .collect();

    for (idx, part_data) in parts_data.iter().enumerate() {
        let part_number = (idx + 1) as i32;

        println!("    Uploading part {} ({} bytes)...", part_number, part_data.len());
        let upload_result = client
            .upload_part()
            .bucket(&bucket)
            .key(object_key)
            .upload_id(upload_id)
            .part_number(part_number)
            .body(part_data.clone())
            .send()
            .await?;

        println!("      ETag: {}", upload_result.etag());
        completed_parts.push(CompletedPart::new(part_number, upload_result.etag()));
    }

    // Step 3: List uploaded parts (optional, for verification)
    println!("  Step 3: Listing uploaded parts...");
    let list_result = client
        .list_parts()
        .bucket(&bucket)
        .key(object_key)
        .upload_id(upload_id)
        .send()
        .await?;

    println!("    Parts count: {}", list_result.parts().len());
    for part in list_result.parts() {
        println!(
            "      Part {}: {} bytes, ETag: {}",
            part.part_number(),
            part.size(),
            part.etag()
        );
    }

    // Step 4: Complete multipart upload
    println!("  Step 4: Completing multipart upload...");
    let complete_result = client
        .complete_multipart_upload()
        .bucket(&bucket)
        .key(object_key)
        .upload_id(upload_id)
        .parts(completed_parts)
        .send()
        .await?;

    println!("  Upload completed!");
    println!("    Location: {}", complete_result.location());
    println!("    ETag: {}", complete_result.etag());

    // Verify the uploaded object
    println!("  Verifying uploaded object...");
    let get_result = client
        .get_object()
        .bucket(&bucket)
        .key(object_key)
        .send()
        .await?;

    let expected_size: usize = parts_data.iter().map(|p| p.len()).sum();
    println!("    Object size: {} bytes (expected: {} bytes)", get_result.body().len(), expected_size);
    assert_eq!(get_result.body().len(), expected_size);

    // Clean up
    println!("  Cleaning up...");
    client
        .delete_object()
        .bucket(&bucket)
        .key(object_key)
        .send()
        .await?;
    println!("  Deleted test object.");

    // ========================================
    // Example 2: Abort multipart upload
    // ========================================
    println!("\n[Example 2] Abort multipart upload...");

    // Initialize a multipart upload
    let initiate_result = client
        .initiate_multipart_upload()
        .bucket(&bucket)
        .key("aborted-upload-test.bin")
        .send()
        .await?;

    let upload_id = initiate_result.upload_id();
    println!("  Upload ID: {}", upload_id);

    // Upload one part
    let part_data: Vec<u8> = (0..10 * 1024).map(|i| (i % 256) as u8).collect();
    client
        .upload_part()
        .bucket(&bucket)
        .key("aborted-upload-test.bin")
        .upload_id(upload_id)
        .part_number(1)
        .body(part_data)
        .send()
        .await?;
    println!("  Uploaded one part.");

    // Abort the upload
    client
        .abort_multipart_upload()
        .bucket(&bucket)
        .key("aborted-upload-test.bin")
        .upload_id(upload_id)
        .send()
        .await?;
    println!("  Aborted multipart upload.");

    // ========================================
    // Example 3: List multipart uploads
    // ========================================
    println!("\n[Example 3] List multipart uploads...");

    // Create a multipart upload to list
    let initiate_result = client
        .initiate_multipart_upload()
        .bucket(&bucket)
        .key("list-uploads-test.bin")
        .send()
        .await?;

    let upload_id = initiate_result.upload_id();
    println!("  Created upload with ID: {}", upload_id);

    // List all in-progress multipart uploads
    let list_result = client
        .list_multipart_uploads()
        .bucket(&bucket)
        .max_uploads(10)
        .send()
        .await?;

    println!("  In-progress uploads count: {}", list_result.uploads().len());
    for upload in list_result.uploads() {
        println!(
            "    Key: {}, UploadId: {}, Initiated: {}",
            upload.key(),
            upload.upload_id(),
            upload.initiated()
        );
    }

    // Clean up
    client
        .abort_multipart_upload()
        .bucket(&bucket)
        .key("list-uploads-test.bin")
        .upload_id(upload_id)
        .send()
        .await?;
    println!("  Cleaned up test upload.");

    println!("\nAll examples completed successfully!");
    Ok(())
}