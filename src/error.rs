use reqwest;
use serde_json;

#[derive(Debug)]
pub enum GroupmeError {
    BotNotFound,
    NoTokenError,
    Unauthorized,
    BadHeaderError(String),
    ReqwestError(reqwest::Error),
    SerdeError(serde_json::Error),
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
