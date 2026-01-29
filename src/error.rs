use reqwest::StatusCode;
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
    ParseOrConvert(String),

    #[error("next position error")]
    NextPosition,

    #[error("serialize error")]
    Serialize {
        raw: String,
        err: serde_xml_rs::Error,
    },
    #[error("unknown data store error")]
    Unknown,
}

pub fn status_to_response<'de, T>(status: StatusCode, text: String) -> Result<T, ObsError>
where
    T: Deserialize<'de>,
{
    match status {
        StatusCode::OK => {
            let r: T = serde_xml_rs::from_str(&text).map_err(|e| ObsError::Serialize {
                raw: text.clone(),
                err: e,
            })?;
            Ok(r)
        }
        StatusCode::FORBIDDEN => match serde_xml_rs::from_str::<ErrorResponse>(&text) {
            Ok(er) => Err(ObsError::Response {
                status: StatusCode::FORBIDDEN,
                message: er.message,
            }),
            Err(e) => Err(ObsError::Response {
                status: StatusCode::FORBIDDEN,
                message: format!("{:?}", e),
            }),
        },
        StatusCode::NOT_FOUND => match serde_xml_rs::from_str::<ErrorResponse>(&text) {
            Ok(er) => Err(ObsError::Response {
                status: StatusCode::NOT_FOUND,
                message: er.message,
            }),
            Err(e) => Err(ObsError::Response {
                status: StatusCode::NOT_FOUND,
                message: format!("{:?}", e),
            }),
        },
        _ => Err(ObsError::Unknown),
    }
}
