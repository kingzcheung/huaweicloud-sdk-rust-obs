use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct CopyObjectResult {
    #[serde(rename = "LastModified")]
    last_modified : String,
    #[serde(rename = "ETag")]
    etag: String,
}