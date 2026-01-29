use std::env;

use huaweicloud_sdk_rust_obs::{bucket::BucketTrait, error::ObsError};

mod common;

use common::*;

#[tokio::test]
async fn test_create_bucket() -> Result<(), ObsError> {
    let obs = create_obs_client()?;
    let bucket = env::var("OBS_BUCKET").unwrap();
    obs.create_bucket(bucket.as_str(), Some("cn-south-1"))
        .await?;
    Ok(())
}

#[tokio::test]
async fn test_list_object() -> Result<(), ObsError> {
    let obs = create_obs_client()?;
    let bucket = env::var("OBS_BUCKET").unwrap();

    println!("bucket:{}", &bucket);

    let bs = obs.list_buckets().await?;

    let bs = bs.buckets.bucket.first().unwrap();
    let res = obs.list_objects(bs.name.as_str(), None, None, None).await?;
    dbg!(res);
    Ok(())
}

#[tokio::test]
async fn test_bucket_location() -> Result<(), ObsError> {
    let obs = create_obs_client()?;
    let bucket = env::var("OBS_BUCKET").unwrap();
    let res = obs.bucket_location(bucket.as_str()).await?;
    dbg!(res);
    Ok(())
}
