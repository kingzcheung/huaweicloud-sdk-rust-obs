use std::env;

use huaweicloud_sdk_rust_obs::{client::{Client, self}, error::ObsError};

pub const DEFAULT_BUCKET_NAME: &str = "ifree-test";

pub fn create_obs_client()->Result<Client,ObsError> {
    dotenvy::dotenv().unwrap();

    let ak = env::var("OBS_AK").unwrap();
    let sk = env::var("OBS_SK").unwrap();
    // println!("ak:{},sk:{}",&ak,&sk);
    let obs = client::Client::builder()
        .endpoint("https://obs.ap-southeast-1.myhuaweicloud.com")
        .security_provider(&ak, &sk) //ifree-test
        .build()?;

    Ok(obs)
}