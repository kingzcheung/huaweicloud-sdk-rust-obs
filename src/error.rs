
use thiserror::Error;


#[derive(Error, Debug)]
pub enum ObsError {
    #[error("request client build fail")]
    Request(#[from] reqwest::Error),

    #[error("unknown data store error")]
    Unknown,
}