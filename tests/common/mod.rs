use std::env;

use huaweicloud_sdk_rust_obs::{
    client::{self, Client},
    error::ObsError,
};

pub fn create_obs_client() -> Result<Client, ObsError> {
    dotenvy::dotenv().unwrap();
    // let bucket = env::var("OBS_BUCKET").unwrap();
    let ak = env::var("OBS_AK").unwrap();
    let sk = env::var("OBS_SK").unwrap();
    let enpoint = env::var("OBS_ENDPOINT").unwrap();
    // println!("ak:{},sk:{}",&ak,&sk);
    let obs = client::Client::builder()
        .endpoint(enpoint.as_str())
        .security_provider(&ak, &sk)
        .build()?;

    Ok(obs)
}
