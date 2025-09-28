//! echo 'OBS_AK=xxxxxxx' > .env
//! echo 'OBS_SK=xxxxxxxxxxxx' >> .env

use std::{env, io::Write};

use huaweicloud_sdk_rust_obs::{error::ObsError, client, object::ObjectTrait};



#[tokio::main]
async fn main()->Result<(), ObsError> {
    dotenvy::dotenv().unwrap();

    let ak = env::var("OBS_AK").unwrap();
    let sk = env::var("OBS_SK").unwrap();
    let bucket = env::var("OBS_BUCKET").unwrap();
    let endpoint = env::var("OBS_ENDPOINT").unwrap();
    println!("bucket:{bucket}");
    // println!("ak:{},sk:{}",&ak,&sk);
    let obs = client::Client::builder()
        .endpoint(endpoint)
        .security_provider(&ak, &sk) //ifree-test
        .build()?;
    let key: &'static str= "test.jpeg";

    let object = include_bytes!("../testdata/test.jpeg");
    obs.put_object(bucket.as_str(), key, object)
        .await?;

    Ok(())
}