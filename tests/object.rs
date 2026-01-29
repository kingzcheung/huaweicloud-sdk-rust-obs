mod common;
use std::env;

use huaweicloud_sdk_rust_obs::{
    error::ObsError, model::delete_object::ResponseMode, object::ObjectTrait,
};

use crate::common::*;

#[tokio::test]
async fn test_put_object() -> Result<(), ObsError> {
    let obs = create_obs_client()?;

    let object = include_bytes!("../testdata/test.jpeg");
    let bucket = env::var("OBS_BUCKET").unwrap();
    obs.put_object(bucket.as_str(), "obs-client-key.jpeg", object)
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_get_object_metadata() -> Result<(), ObsError> {
    let obs = create_obs_client()?;

    let bucket = env::var("OBS_BUCKET").unwrap();
    let meta = obs
        .get_object_metadata(bucket.as_str(), "obs-client-key.jpeg")
        .await?;
    println!("{:?}", meta);
    Ok(())
}

#[tokio::test]
async fn test_copy_object() -> Result<(), ObsError> {
    let obs = create_obs_client()?;
    let src = "/obs-client-key.jpeg";
    let dest = "obs-client-key_copy.jpeg";
    let bucket = env::var("OBS_BUCKET").unwrap();
    obs.copy_object(bucket.as_str(), src, dest).await?;
    obs.delete_object(bucket.as_str(), dest).await?;
    Ok(())
}

#[tokio::test]
async fn test_get_object() -> Result<(), ObsError> {
    let obs = create_obs_client()?;
    let key = "obs-client-key.jpeg";
    let bucket = env::var("OBS_BUCKET").unwrap();
    let data = obs.get_object(bucket.as_str(), key).await?;
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
    let bucket = env::var("OBS_BUCKET").unwrap();
    let next_position = obs
        .append_object(bucket.as_str(), key, appended.as_bytes(), position)
        .await?;
    assert!(next_position.is_some());
    dbg!(next_position);
    let n2 = obs
        .append_object(
            bucket.as_str(),
            key,
            appended2.as_bytes(),
            next_position.unwrap(),
        )
        .await?;
    assert!(n2.is_some());
    let data = obs.get_object(bucket.as_str(), key).await?;
    assert_eq!(data.len(), appended.len() + appended2.len());
    // obs.delete_object(DEFAULT_BUCKET_NAME, key).await?;
    Ok(())
}

#[tokio::test]
async fn test_delete_objects() -> Result<(), ObsError> {
    let objects = vec![
        "obs-client-test-delete-object.txt",
        "obs-client-test-delete-object2.txt",
    ];
    let obs = create_obs_client()?;
    let bucket = env::var("OBS_BUCKET").unwrap();
    for obj in &objects {
        obs.put_object(bucket.as_str(), obj, b"test delete text")
            .await?;
    }

    obs.delete_objects(bucket.as_str(), objects, ResponseMode::Verbose)
        .await?;

    Ok(())
}
