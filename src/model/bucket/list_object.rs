use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ListObjectsResponse {
    #[serde(rename = "ListBucketResult")]
    list_bucket_result: ListBucketResult,
}

#[derive(Serialize, Deserialize)]
pub struct ListBucketResult {
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "Prefix")]
    prefix: String,

    #[serde(rename = "Marker")]
    marker: String,

    #[serde(rename = "MaxKeys")]
    max_keys: String,

    #[serde(rename = "IsTruncated")]
    is_truncated: String,

    #[serde(rename = "Contents")]
    contents: Contents,
}

#[derive(Serialize, Deserialize)]
pub struct Contents {
    #[serde(rename = "Key")]
    key: String,

    #[serde(rename = "LastModified")]
    last_modified: String,

    #[serde(rename = "ETag")]
    e_tag: String,

    #[serde(rename = "Size")]
    size: String,

    #[serde(rename = "Owner")]
    owner: Owner,

    #[serde(rename = "StorageClass")]
    storage_class: String,
}

#[derive(Serialize, Deserialize)]
pub struct Owner {
    #[serde(rename = "ID")]
    id: String,

    #[serde(rename = "DisplayName")]
    display_name: String,
}