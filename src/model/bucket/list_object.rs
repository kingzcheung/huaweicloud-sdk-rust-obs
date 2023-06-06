use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize)]
pub struct ListBucketResult {
    #[serde(rename = "Name")]
    name: String,

    #[serde(rename = "NextMarker")]
    next_marker: String,

    #[serde(rename = "MaxKeys")]
    max_keys: String,

    #[serde(rename = "IsTruncated")]
    is_truncated: String,

    #[serde(rename = "Contents")]
    contents: Vec<Content>,
}

#[derive(Serialize, Deserialize)]
pub struct Content {
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
    storage_class: StorageClass,
}

#[derive(Serialize, Deserialize)]
pub struct Owner {
    #[serde(rename = "ID")]
    id: Id,
}

#[derive(Serialize, Deserialize)]
pub enum Id {
    #[serde(rename = "0ac96b898e800f220f36c00d9687b180")]
    The0Ac96B898E800F220F36C00D9687B180,
}

#[derive(Serialize, Deserialize)]
pub enum StorageClass {
    #[serde(rename = "STANDARD")]
    Standard,
}