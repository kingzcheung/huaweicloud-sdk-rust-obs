//! Common test utilities

use std::env;

use huaweicloud_sdk_rust_obs::{Client, Config, ObsError};

pub fn setup() -> Result<Client, ObsError> {
    dotenvy::dotenv().ok();

    let access_key_id = env::var("OBS_ACCESS_KEY_ID").expect("OBS_ACCESS_KEY_ID must be set");
    let secret_access_key = env::var("OBS_SECRET_ACCESS_KEY").expect("OBS_SECRET_ACCESS_KEY must be set");
    let endpoint = env::var("OBS_ENDPOINT").expect("OBS_ENDPOINT must be set");

    let config = Config::builder()
        .access_key(access_key_id, secret_access_key)
        .endpoint(&endpoint)
        .build()?;

    Client::from_config(config)
}
