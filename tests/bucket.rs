use huaweicloud_sdk_rust_obs::{error::ObsError, bucket::BucketTrait};

mod common;

use common::*;

#[tokio::test]
async fn test_list_object()->Result<(), ObsError>{
    let obs = create_obs_client()?;

    let res = obs.list_objects(DEFAULT_BUCKET_NAME, None, None, None).await?;
    Ok(())
}