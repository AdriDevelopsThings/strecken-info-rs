#[derive(Debug)]
pub enum StreckenInfoError {
    /// A HTTP error occured while requesting
    RequestError(reqwest::Error),
    JsonError(serde_json::Error),
    /// This error could happen if the server doesn't respond with the right response to a request
    InvalidResponse,
    /// The server sends an error field which is `OK` by default, if it's this error will be thrown
    ResponseError(String),
}

impl From<reqwest::Error> for StreckenInfoError {
    fn from(value: reqwest::Error) -> Self {
        Self::RequestError(value)
    }
}

impl From<serde_json::Error> for StreckenInfoError {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonError(value)
    }
}
