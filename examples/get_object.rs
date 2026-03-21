//! Example: Download an object from OBS
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

    // Create a client using AWS SDK style
    let config = Config::builder()
        .access_key(access_key_id, secret_access_key)
        .endpoint(&endpoint)
        .build()?;

    let client = Client::from_config(config)?;

    // Download an object using fluent builder API
    let key = "test.jpeg";

    let result = client.get_object().bucket(&bucket).key(key).send().await?;

    println!("Downloaded object: {}", key);
    println!("Content length: {:?}", result.content_length());
    println!("Content type: {:?}", result.content_type());
    println!("ETag: {:?}", result.etag());

    // Get the body bytes
    let bytes = result.body();
    println!("Body size: {} bytes", bytes.len());

    Ok(())
}
