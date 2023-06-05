use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct CopyObjectResponse {
    #[serde(rename = "LastModified")]
    last_modified : String,
    #[serde(rename = "ETag")]
    etag: String,
}