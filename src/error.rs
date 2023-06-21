use reqwest::{StatusCode};
use serde::Deserialize;
use thiserror::Error;

use crate::model::ErrorResponse;

#[derive(Error, Debug)]
pub enum ObsError {
    #[error("request client build fail")]
    Request(#[from] reqwest::Error),

    #[error("ak or sk not provided")]
    Security,

    #[error("operation is not valid, status:{status:?}, message:{message:?}")]
    Response { status: StatusCode, message: String },

    #[error("parse or convert json error")]
    ParseOrConvert,

    #[error("next position error")]
    NextPosition,

    #[error("serialize error")]
    Serialize(#[from] serde_xml_rs::Error),
    #[error("unknown data store error")]
    Unknown,
}

pub fn status_to_response<'de, T>(status: StatusCode, text: String) -> Result<T, ObsError>
where
    T: Deserialize<'de>,
{
    match status {
        StatusCode::OK => {
            let r: T = serde_xml_rs::from_str(&text)?;
            Ok(r)
        }
        StatusCode::FORBIDDEN => {
            let er: ErrorResponse = serde_xml_rs::from_str(&text)?;
            Err(ObsError::Response {
                status: StatusCode::FORBIDDEN,
                message: er.message,
            })
        }
        _ => Err(ObsError::Unknown),
    }
}
