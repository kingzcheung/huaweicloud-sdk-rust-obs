use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct CreateBucketRequest {
    #[serde(rename = "CreateBucketConfiguration")]
    pub create_bucket_configuration: CreateBucketConfiguration,
}

impl CreateBucketRequest {
    pub fn new<S: ToString>(location: S) -> Self {
        let create_bucket_configuration = CreateBucketConfiguration {
            location: location.to_string(),
        };

        Self {
            create_bucket_configuration,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateBucketConfiguration {
    #[serde(rename = "Location")]
    location: String,
}
