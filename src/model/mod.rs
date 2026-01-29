use serde::{Deserialize, Serialize};

pub mod bucket;
pub mod delete_object;
pub mod object;

// #[derive(Serialize, Deserialize,Debug)]
// pub struct ErrorResponse {
//     #[serde(rename = "Error")]
//     pub error: Error,
// }

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResponse {
    #[serde(rename = "Code")]
    pub code: String,

    #[serde(rename = "Message")]
    pub message: String,

    #[serde(rename = "RequestId")]
    pub request_id: String,

    #[serde(rename = "HostId")]
    pub host_id: String,

    #[serde(rename = "AccessKeyId", skip_serializing_if = "Option::is_none")]
    pub access_key_id: Option<String>,

    #[serde(rename = "SignatureProvided", skip_serializing_if = "Option::is_none")]
    pub signature_provided: Option<String>,

    #[serde(rename = "StringToSign", skip_serializing_if = "Option::is_none")]
    pub string_to_sign: Option<String>,

    #[serde(rename = "StringToSignBytes", skip_serializing_if = "Option::is_none")]
    pub string_to_sign_bytes: Option<String>,
}
