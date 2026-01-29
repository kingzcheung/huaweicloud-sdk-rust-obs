use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ListBucketResult {
    #[serde(rename = "Name")]
    pub name: String,

    #[serde(rename = "Prefix")]
    pub prefix: Option<String>,

    #[serde(rename = "Delimiter")]
    pub delimiter: Option<String>,

    #[serde(rename = "EncodingType")]
    pub encoding_type: Option<String>,

    #[serde(rename = "NextMarker")]
    pub next_marker: Option<String>,

    #[serde(rename = "MaxKeys")]
    pub max_keys: String,

    #[serde(rename = "IsTruncated")]
    pub is_truncated: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "Contents")]
    pub contents: Option<Vec<Content>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "CommonPrefixes")]
    pub common_prefixes: Option<Vec<CommonPrefixes>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CommonPrefixes {
    #[serde(rename = "Prefix")]
    pub prefix: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Content {
    #[serde(rename = "Key")]
    pub key: String,

    #[serde(rename = "LastModified")]
    pub last_modified: String,

    #[serde(rename = "ETag")]
    pub e_tag: String,

    #[serde(rename = "Size")]
    pub size: usize,

    #[serde(rename = "Owner")]
    pub owner: Owner,

    #[serde(rename = "StorageClass")]
    pub storage_class: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Owner {
    #[serde(rename = "ID")]
    id: String,
}
