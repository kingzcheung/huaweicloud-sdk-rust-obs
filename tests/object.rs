mod common;
use huaweicloud_sdk_rust_obs::{error::ObsError, object::ObjectTrait};

use crate::common::*;

#[tokio::test]
async fn test_put_object() -> Result<(), ObsError> {
    let obs = create_obs_client()?;

    let object = include_bytes!("../testdata/test.jpeg");
    obs.put_object(DEFAULT_BUCKET_NAME, "obs-client-key.jpeg", object)
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_get_object_metadata() -> Result<(), ObsError> {
    let obs = create_obs_client()?;

    let meta = obs
        .get_object_metadata(DEFAULT_BUCKET_NAME, "obs-client-key.jpeg")
        .await?;
    dbg!(meta);
    Ok(())
}

#[tokio::test]
async fn test_copy_object()->Result<(), ObsError> {
    let obs = create_obs_client()?;
    let src = "/obs-client-key.jpeg";
    let dest = "obs-client-key_copy.jpeg";
    obs.copy_object(DEFAULT_BUCKET_NAME, src, dest).await?;
    obs.delete_object(DEFAULT_BUCKET_NAME, dest).await?;
    Ok(())
}

#[tokio::test]
async fn test_get_object()->Result<(),ObsError> {
    let obs = create_obs_client()?;
    let key = "obs-client-key.jpeg";
    let data = obs.get_object(DEFAULT_BUCKET_NAME, key).await?;
    dbg!(data);
    Ok(())
}