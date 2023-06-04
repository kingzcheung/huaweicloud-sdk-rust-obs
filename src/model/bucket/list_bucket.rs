use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ListAllMyBuckets {
    #[serde(rename = "ListAllMyBucketsResult")]
    list_all_my_buckets_result: ListAllMyBucketsResult,
}

#[derive(Serialize, Deserialize)]
pub struct ListAllMyBucketsResult {
    #[serde(rename = "Owner")]
    owner: Owner,

    #[serde(rename = "Buckets")]
    buckets: Buckets,
}

#[derive(Serialize, Deserialize)]
pub struct Buckets {
    #[serde(rename = "Bucket")]
    bucket: Vec<Bucket>,
}

#[derive(Serialize, Deserialize)]
pub struct Bucket {
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "CreationDate")]
    creation_date: String,

    #[serde(rename = "Location")]
    location: String,

    #[serde(rename = "BucketType")]
    bucket_type: String,
}

#[derive(Serialize, Deserialize)]
pub struct Owner {
    #[serde(rename = "ID")]
    id: String,
}



