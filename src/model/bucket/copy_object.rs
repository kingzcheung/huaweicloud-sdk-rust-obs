use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CopyObjectResult {
    #[serde(rename = "LastModified")]
    pub last_modified: String,
    #[serde(rename = "ETag")]
    pub etag: String,
}
