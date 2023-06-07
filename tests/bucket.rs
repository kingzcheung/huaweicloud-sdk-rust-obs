use huaweicloud_sdk_rust_obs::{error::ObsError, bucket::BucketTrait};

mod common;

use common::*;

#[tokio::test]
async fn test_list_object()->Result<(), ObsError>{
    let obs = create_obs_client()?;

    let _res = obs.list_objects(DEFAULT_BUCKET_NAME, None, None, None).await?;
    let _ = obs.list_objects(DEFAULT_BUCKET_NAME, None, Some("test"), None).await?;
    Ok(())
}