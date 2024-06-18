#[derive(Debug)]
pub enum StreckenInfoError {
    /// A HTTP error occured while requesting
    RequestError(reqwest::Error),
    /// A websocket error occured
    WebsocketError(tokio_tungstenite::tungstenite::Error),
    /// The websocket sent no revision
    WebSocketNoRevisionError,
    JsonError(serde_json::Error),
    /// This error could happen if the server doesn't respond with the right response to a request
    InvalidResponse,
    /// The server sends an error field which is `OK` by default, if it's this error will be thrown
    ResponseError(String),
    /// There are multiple references in the response, this error will be thrown if they are broken
    ReferenceError,
}

impl From<reqwest::Error> for StreckenInfoError {
    fn from(value: reqwest::Error) -> Self {
        Self::RequestError(value)
    }
}

impl From<tokio_tungstenite::tungstenite::Error> for StreckenInfoError {
    fn from(value: tokio_tungstenite::tungstenite::Error) -> Self {
        Self::WebsocketError(value)
    }
}

impl From<serde_json::Error> for StreckenInfoError {
    fn from(value: serde_json::Error) -> Self {
        Self::JsonError(value)
    }
}
