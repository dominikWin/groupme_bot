use reqwest;
use serde_json;

/// The error type for any issues occuring while interacting with the Groupme API.
#[derive(Debug)]
pub enum GroupmeError {
    /// Caused by calling a method that requires an API token without providing one.
    ///
    /// When managing bots (creating or deleting) make sure to provide the
    /// `Groupme` a valid token.
    NoTokenError,
    /// Any request that caused the API to return an Unauthorized header.
    ///
    /// This is usually caused by an invalid API token.
    Unauthorized,
    /// The API returned an unexpected HTTP header.
    ///
    /// This is usually caused by not defining a proper bot_id or group_id.
    BadHeaderError(reqwest::StatusCode),
    /// Error communicating with the API.
    ReqwestError(reqwest::Error),
    /// Error parsing returned JSON values.
    SerdeError(serde_json::Error),
    /// Caused by an issue with interacting with the data returned from the API.
    ///
    /// This is used when JSON data is parsed but is not valid.
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
