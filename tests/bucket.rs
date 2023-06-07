use huaweicloud_sdk_rust_obs::{error::ObsError, bucket::BucketTrait};

mod common;

use common::*;

#[tokio::test]
async fn test_list_object()->Result<(), ObsError>{
    let obs = create_obs_client()?;

    // let _res = obs.list_objects(DEFAULT_BUCKET_NAME, None, None, None).await?;
    let res = obs.list_objects(DEFAULT_BUCKET_NAME,Some("obs-client-key.jpeg"), None,  Some(10)).await?;
    dbg!(res);
    Ok(())
}