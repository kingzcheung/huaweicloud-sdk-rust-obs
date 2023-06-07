use serde::{Deserialize, Serialize};


#[derive(Serialize, Deserialize,Debug)]
pub struct ListBucketResult {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "NextMarker")]
    pub next_marker: Option<String>,

    #[serde(rename = "MaxKeys")]
    pub max_keys: String,

    #[serde(rename = "IsTruncated")]
    pub is_truncated: String,

    #[serde(rename = "Contents")]
    pub contents: Vec<Content>,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Content {
    #[serde(rename = "Key")]
    pub key: String,

    #[serde(rename = "LastModified")]
    pub last_modified: String,

    #[serde(rename = "ETag")]
    pub e_tag: String,

    #[serde(rename = "Size")]
    pub size: String,

    #[serde(rename = "Owner")]
    pub owner: Owner,

    #[serde(rename = "StorageClass")]
    pub storage_class: StorageClass,
}

#[derive(Serialize, Deserialize,Debug)]
pub struct Owner {
    #[serde(rename = "ID")]
    id: Id,
}

#[derive(Serialize, Deserialize,Debug)]
pub enum Id {
    #[serde(rename = "0ac96b898e800f220f36c00d9687b180")]
    The0Ac96B898E800F220F36C00D9687B180,
}

#[derive(Serialize, Deserialize,Debug)]
pub enum StorageClass {
    #[serde(rename = "STANDARD")]
    Standard,
}