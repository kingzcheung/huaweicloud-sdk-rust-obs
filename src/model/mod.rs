use serde::{Serialize, Deserialize};

pub mod object;
pub mod bucket;

// #[derive(Serialize, Deserialize,Debug)]
// pub struct ErrorResponse {
//     #[serde(rename = "Error")]
//     pub error: Error,
// }

#[derive(Serialize, Deserialize,Debug)]
pub struct ErrorResponse {
    #[serde(rename = "Code")]
    pub code: String,

    #[serde(rename = "Message")]
    pub message: String,

    #[serde(rename = "RequestId")]
    pub request_id: String,

    #[serde(rename = "HostId")]
    pub host_id: String,

    #[serde(rename = "AccessKeyId")]
    pub access_key_id: String,

    #[serde(rename = "SignatureProvided")]
    pub signature_provided: String,

    #[serde(rename = "StringToSign")]
    pub string_to_sign: String,

    #[serde(rename = "StringToSignBytes")]
    pub string_to_sign_bytes: String,
}
