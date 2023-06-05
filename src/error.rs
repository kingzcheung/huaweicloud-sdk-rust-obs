use reqwest::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ObsError {
    #[error("request client build fail")]
    Request(#[from] reqwest::Error),

    #[error("ak or sk not provided")]
    Security,

    #[error("operation is not valid, status:{status:?}, message:{message:?}")]
    Response {
        status: StatusCode,
        message: String,
    },

    #[error("parse or convert json error")]
    ParseOrConvert,

    #[error("serialize error")]
    Serialize(#[from] serde_xml_rs::Error),
    #[error("unknown data store error")]
    Unknown,
}