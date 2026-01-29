use serde::{Deserialize, Serialize};

pub type NextPosition = Option<u64>;

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectMeta {
    #[serde(rename = "server")]
    server: String,

    #[serde(rename = "x-obs-request-id")]
    x_obs_request_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "x-reserved-indicator")]
    x_reserved_indicator: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "accept-ranges")]
    accept_ranges: Option<String>,

    #[serde(rename = "etag")]
    etag: String,

    #[serde(rename = "last-modified")]
    last_modified: String,

    #[serde(rename = "content-type")]
    content_type: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "x-obs-tagging-count")]
    x_obs_tagging_count: Option<String>,

    #[serde(rename = "x-obs-id-2")]
    x_obs_id_2: String,

    #[serde(rename = "date")]
    date: String,

    #[serde(rename = "content-length")]
    content_length: String,
}
