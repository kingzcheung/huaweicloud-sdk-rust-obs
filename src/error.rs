use thiserror::Error;

#[derive(Error, Debug)]
pub enum ObsError {
    #[error("request client build fail")]
    Request(#[from] reqwest::Error),

    #[error("ak or sk not provided")]
    Security,

    #[error("parse or convert json error")]
    ParseOrConvert,

    #[error("unknown data store error")]
    Unknown,
}