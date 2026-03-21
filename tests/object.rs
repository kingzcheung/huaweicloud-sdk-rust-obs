//! Tests for object operations

mod common;

use huaweicloud_sdk_rust_obs::ObsError;
use std::env;

#[tokio::test]
async fn test_put_and_get_object() -> Result<(), ObsError> {
    let obs = common::setup()?;
    let bucket = env::var("OBS_BUCKET").expect("OBS_BUCKET must be set");
    let key = "test-object.txt";
    let content = b"Hello, OBS!";

    // Put object
    let put_result = obs.put_object()
        .bucket(&bucket)
        .key(key)
        .body(content.to_vec())
        .content_type("text/plain")
        .send()
        .await?;

    println!("Put object: {}", key);
    if let Some(etag) = put_result.etag() {
        println!("ETag: {}", etag);
    }

    // Get object
    let get_result = obs.get_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    println!("Get object: {}", key);
    println!("Content length: {:?}", get_result.content_length());
    println!("Content type: {:?}", get_result.content_type());

    let body = get_result.body();
    assert_eq!(body.as_ref(), content);

    // Clean up
    obs.delete_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    println!("Deleted object: {}", key);

    Ok(())
}

#[tokio::test]
async fn test_head_object() -> Result<(), ObsError> {
    let obs = common::setup()?;
    let bucket = env::var("OBS_BUCKET").expect("OBS_BUCKET must be set");
    let key = "test-head-object.txt";
    let content = b"Test content for head operation";

    // Put object
    obs.put_object()
        .bucket(&bucket)
        .key(key)
        .body(content.to_vec())
        .content_type("text/plain")
        .send()
        .await?;

    // Head object
    let head_result = obs.head_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    println!("Head object: {}", key);
    println!("Content length: {:?}", head_result.content_length());
    println!("Content type: {:?}", head_result.content_type());
    println!("ETag: {:?}", head_result.etag());

    // Clean up
    obs.delete_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_copy_object() -> Result<(), ObsError> {
    let obs = common::setup()?;
    let bucket = env::var("OBS_BUCKET").expect("OBS_BUCKET must be set");
    let src_key = "test-copy-source.txt";
    let dest_key = "test-copy-dest.txt";
    let content = b"Content to be copied";

    // Put source object
    obs.put_object()
        .bucket(&bucket)
        .key(src_key)
        .body(content.to_vec())
        .send()
        .await?;

    // Copy object
    let copy_result = obs.copy_object()
        .bucket(&bucket)
        .key(dest_key)
        .copy_source(format!("{}/{}", bucket, src_key))
        .send()
        .await?;

    println!("Copied object from {} to {}", src_key, dest_key);
    println!("ETag: {}", copy_result.etag());

    // Verify the copy
    let get_result = obs.get_object()
        .bucket(&bucket)
        .key(dest_key)
        .send()
        .await?;

    assert_eq!(get_result.body().as_ref(), content);

    // Clean up
    obs.delete_object()
        .bucket(&bucket)
        .key(src_key)
        .send()
        .await?;

    obs.delete_object()
        .bucket(&bucket)
        .key(dest_key)
        .send()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_append_object() -> Result<(), ObsError> {
    let obs = common::setup()?;
    let bucket = env::var("OBS_BUCKET").expect("OBS_BUCKET must be set");
    let key = "test-append-object.txt";

    // First append
    let append1 = obs.append_object()
        .bucket(&bucket)
        .key(key)
        .position(0)
        .body(b"Hello, ".to_vec())
        .send()
        .await?;

    println!("First append, next position: {:?}", append1.next_position());

    // Second append
    let next_pos = append1.next_position().unwrap_or(7);
    let append2 = obs.append_object()
        .bucket(&bucket)
        .key(key)
        .position(next_pos)
        .body(b"World!".to_vec())
        .send()
        .await?;

    println!("Second append, next position: {:?}", append2.next_position());

    // Verify the content
    let get_result = obs.get_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    let body = get_result.body();
    assert_eq!(body.as_ref(), b"Hello, World!");

    // Clean up
    obs.delete_object()
        .bucket(&bucket)
        .key(key)
        .send()
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_delete_objects() -> Result<(), ObsError> {
    let obs = common::setup()?;
    let bucket = env::var("OBS_BUCKET").expect("OBS_BUCKET must be set");

    // Create multiple objects
    let keys = vec![
        "test-delete-1.txt".to_string(),
        "test-delete-2.txt".to_string(),
        "test-delete-3.txt".to_string(),
    ];

    for key in &keys {
        obs.put_object()
            .bucket(&bucket)
            .key(key)
            .body(b"test content".to_vec())
            .send()
            .await?;
    }

    // Delete multiple objects
    obs.delete_objects()
        .bucket(&bucket)
        .keys(keys.clone())
        .send()
        .await?;

    println!("Deleted {} objects", keys.len());

    // Verify objects are deleted
    for key in &keys {
        let result = obs.head_object()
            .bucket(&bucket)
            .key(key)
            .send()
            .await;

        // Should fail because object doesn't exist
        assert!(result.is_err());
    }

    Ok(())
}
