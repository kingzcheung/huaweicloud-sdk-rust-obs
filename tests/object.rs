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
async fn test_copy_object() -> Result<(), ObsError> {
    let obs = create_obs_client()?;
    let src = "/obs-client-key.jpeg";
    let dest = "obs-client-key_copy.jpeg";
    obs.copy_object(DEFAULT_BUCKET_NAME, src, dest).await?;
    obs.delete_object(DEFAULT_BUCKET_NAME, dest).await?;
    Ok(())
}

#[tokio::test]
async fn test_get_object() -> Result<(), ObsError> {
    let obs = create_obs_client()?;
    let key = "obs-client-key.jpeg";
    let data = obs.get_object(DEFAULT_BUCKET_NAME, key).await?;
    dbg!(data);
    Ok(())
}

#[tokio::test]
async fn test_append_object() -> Result<(), ObsError> {
    let obs = create_obs_client()?;
    let key = "obs-client-append-key.txt";
    let appended = "hello world";
    let appended2 = ",cc";
    let position = 0;
    let next_position = obs
        .append_object(DEFAULT_BUCKET_NAME, key, appended.as_bytes(), position)
        .await?;
    assert!(next_position.is_some());
    dbg!(next_position);
    let n2 = obs
        .append_object(
            DEFAULT_BUCKET_NAME,
            key,
            appended2.as_bytes(),
            next_position.unwrap(),
        )
        .await?;
    assert!(n2.is_some());
    let data = obs.get_object(DEFAULT_BUCKET_NAME, key).await?;
    assert_eq!(data.len(), appended.len() + appended2.len());
    // obs.delete_object(DEFAULT_BUCKET_NAME, key).await?;
    Ok(())
}
