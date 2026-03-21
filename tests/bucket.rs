//! Tests for bucket operations

mod common;

use huaweicloud_sdk_rust_obs::ObsError;

#[tokio::test]
async fn test_list_buckets() -> Result<(), ObsError> {
    let obs = common::setup()?;

    let result = obs.list_buckets().send().await?;

    println!("Found {} buckets", result.buckets().len());
    for bucket in result.buckets() {
        println!("  - {} ({})", bucket.name(), bucket.location());
    }

    Ok(())
}

#[tokio::test]
async fn test_create_and_delete_bucket() -> Result<(), ObsError> {
    let obs = common::setup()?;
    let bucket_name = format!("test-bucket-{}", chrono::Utc::now().timestamp());

    // Create bucket
    obs.create_bucket()
        .bucket(&bucket_name)
        .location_constraint("cn-north-4")
        .send()
        .await?;

    println!("Created bucket: {}", bucket_name);

    // Delete bucket
    obs.delete_bucket()
        .bucket(&bucket_name)
        .send()
        .await?;

    println!("Deleted bucket: {}", bucket_name);

    Ok(())
}

#[tokio::test]
async fn test_list_objects() -> Result<(), ObsError> {
    let obs = common::setup()?;

    // First list buckets
    let buckets = obs.list_buckets().send().await?;

    if buckets.buckets().is_empty() {
        println!("No buckets found, skipping test");
        return Ok(());
    }

    let bucket_name = buckets.buckets()[0].name();

    // List objects in the first bucket
    let result = obs.list_objects()
        .bucket(bucket_name)
        .max_keys(10)
        .send()
        .await?;

    println!("Objects in bucket '{}':", bucket_name);
    for obj in result.contents() {
        println!("  - {} ({} bytes)", obj.key(), obj.size());
    }

    Ok(())
}

#[tokio::test]
async fn test_get_bucket_location() -> Result<(), ObsError> {
    let obs = common::setup()?;

    // First list buckets
    let buckets = obs.list_buckets().send().await?;

    if buckets.buckets().is_empty() {
        println!("No buckets found, skipping test");
        return Ok(());
    }

    let bucket_name = buckets.buckets()[0].name();

    // Get bucket location
    let result = obs.get_bucket_location()
        .bucket(bucket_name)
        .send()
        .await?;

    println!("Bucket '{}' location: {}", bucket_name, result.location());

    Ok(())
}
