use reqwest;
use serde_json;

#[derive(Debug)]
pub enum GroupmeError {
    NoTokenError,
    Unauthorized,
    BadHeaderError(reqwest::StatusCode),
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::Error),
    GenericError,
}

impl From<reqwest::Error> for GroupmeError {
    fn from(error: reqwest::Error) -> Self {
        GroupmeError::ReqwestError(error)
    }
}

impl From<serde_json::Error> for GroupmeError {
    fn from(error: serde_json::Error) -> Self {
        GroupmeError::SerdeError(error)
    }
}
