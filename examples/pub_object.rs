//! Example: Upload an object to OBS
//!
//! Usage:
//! 1. Create a .env file with:
//!    OBS_ACCESS_KEY_ID=your_access_key
//!    OBS_SECRET_ACCESS_KEY=your_secret_key
//!    OBS_BUCKET=your_bucket_name
//!    OBS_ENDPOINT=obs.cn-north-4.myhuaweicloud.com

use std::env;

use huaweicloud_sdk_rust_obs::{Client, Config, ObsError};

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

    // Create a client using AWS SDK style
    let config = Config::builder()
        .access_key(access_key_id, secret_access_key)
        .endpoint(&endpoint)
        .build()?;

    let client = Client::from_config(config)?;

    // Upload an object using fluent builder API
    let key = "test.jpeg";
    let object = include_bytes!("../testdata/test.jpeg");

    let result = client
        .put_object()
        .bucket(&bucket)
        .key(key)
        .body(object.to_vec())
        .content_type("image/jpeg")
        .send()
        .await?;

    println!("Uploaded object: {}", key);
    if let Some(etag) = result.etag() {
        println!("ETag: {}", etag);
    }

    Ok(())
}
