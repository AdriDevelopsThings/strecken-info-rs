use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::error::StreckenInfoError;

const WEBSOCKET_PATH: &str = "wss://strecken-info.de/api/websocket";

#[derive(Deserialize)]
struct RevisionJson {
    revision: u32,
}

pub async fn get_revision() -> Result<u32, StreckenInfoError> {
    let (mut ws, _) = connect_async(WEBSOCKET_PATH).await?;
    ws.send(Message::text("{\"type\":\"HANDSHAKE\",\"revision\":null}"))
        .await?;
    let msg = ws
        .next()
        .await
        .ok_or(StreckenInfoError::WebSocketNoRevisionError)??;
    ws.close(None).await?;
    let json: RevisionJson = serde_json::from_slice(&msg.into_data()).unwrap();
    Ok(json.revision)
}
