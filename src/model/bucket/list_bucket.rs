use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListAllMyBucketsResult {
    #[serde(rename = "Owner")]
    pub owner: Owner,

    #[serde(rename = "Buckets")]
    pub buckets: Buckets,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Buckets {
    #[serde(rename = "Bucket")]
    pub bucket: Vec<Bucket>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Bucket {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "CreationDate")]
    pub creation_date: String,

    #[serde(rename = "Location")]
    pub location: String,

    #[serde(rename = "BucketType")]
    pub bucket_type: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Owner {
    #[serde(rename = "ID")]
    pub id: String,
}
