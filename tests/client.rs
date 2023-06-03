use std::env;

use huaweicloud_sdk_rust_obs::{client, error::ObsError};

#[tokio::test]
async fn test_put_object() -> Result<(), ObsError> {
    dotenvy::dotenv().unwrap();

    let ak = env::var("OBS_AK").unwrap();
    let sk = env::var("OBS_SK").unwrap();
    let obs = client::Client::builder()
        .endpoint("https://obs.ap-southeast-1.myhuaweicloud.com")
        .security_provider(&ak, &sk) //ifree-test
        .build()?;

    let object = include_bytes!("../testdata/test.jpeg");
    obs.put_object("ifree-test", "obs-client-key.jpeg", object)
        .await?;

    Ok(())
}

#[tokio::test]
async fn test_get_object_metadata() -> Result<(), ObsError> {
    dotenvy::dotenv().unwrap();

    let ak = env::var("OBS_AK").unwrap();
    let sk = env::var("OBS_SK").unwrap();
    let obs = client::Client::builder()
        .endpoint("https://obs.ap-southeast-1.myhuaweicloud.com")
        .security_provider(&ak, &sk) //ifree-test
        .build()?;

    let meta = obs
        .get_object_metadata("ifree-test", "obs-client-key.jpeg")
        .await?;
    dbg!(meta);
    Ok(())
}
