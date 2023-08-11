use serde::{ Serialize, Deserialize };

// #[derive(Debug, Serialize, Deserialize, PartialEq)]
// pub struct DeleteRequest {
//     #[serde(rename = "Delete")]
//     pub delete: Delete,
// }

pub trait Boolean {
    fn to_bool(&self) -> bool;
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum ResponseMode {
    Quiet,
    Verbose
}

impl Boolean for ResponseMode {
    fn to_bool(&self) -> bool {
        match self {
            ResponseMode::Quiet => true,
            ResponseMode::Verbose => false,
        }
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Delete {
    #[serde(rename = "Quiet")]
    pub quiet: bool,
    #[serde(rename = "$value")]
    pub item: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Item {
    Object(Object),
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Object {
    #[serde(rename = "Key")]
    pub key_name: String,
    #[serde(rename = "VersionId")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DeleteResult {
    #[serde(rename = "Deleted")]
    pub deleted: Deleted,
    #[serde(rename = "Error")]
    pub error: Error,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Deleted {
    #[serde(rename = "Key")]
    pub key_name: String,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Error {
    #[serde(rename = "Key")]
    pub key_name: String,
    #[serde(rename = "Code")]
    pub code: String,
    #[serde(rename = "Message")]
    pub message: String,
}
