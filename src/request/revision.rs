//! Revisions are like versions of disruptions. To get the disruptions you need a revision.
//! ```no_run
//! use strecken_info::revision::get_revision;
//!
//! #[tokio::main]
//! async fn main() {
//!     let revision: u32 = get_revision().await.unwrap();
//! }
//! ```
//! If you want to wait for a new revision try this:
//! ```no_run
//! use strecken_info::revision::RevisionContext;
//!
//! #[tokio::main]
//! async fn main() {
//!     let mut ctx = RevisionContext::connect().await.unwrap();
//!     let first_revision: u32 = ctx.get_first_revision().await.unwrap();
//!     println!("First revision: {first_revision}");
//!     loop {
//!         let revision = ctx.wait_for_new_revision().await.unwrap();
//!         println!("Got new revision: {revision}");
//!     }
//! }
//! ```

use futures_util::{SinkExt, StreamExt};
use serde::Deserialize;
use tokio::net::TcpStream;
use tokio_tungstenite::{connect_async, tungstenite::Message, MaybeTlsStream, WebSocketStream};

use crate::error::StreckenInfoError;

const WEBSOCKET_PATH: &str = "wss://strecken-info.de/api/websocket";

pub struct RevisionContext {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
}

#[derive(Deserialize)]
struct FirstRevisionJson {
    revision: u32,
}

#[derive(Deserialize)]
#[serde(tag = "type")]
enum UpdateJson {
    #[serde(alias = "NEW_REVISION")]
    NewRevision {
        revision: UpdateRevisionJson,
    },
    Other(()),
}

#[derive(Deserialize)]
struct UpdateRevisionJson {
    #[serde(alias = "nummer")]
    number: u32,
    #[serde(alias = "stoerungen")]
    disruptions: Vec<serde_json::Value>,
}

impl RevisionContext {
    pub async fn connect() -> Result<Self, StreckenInfoError> {
        let (ws, _) = connect_async(WEBSOCKET_PATH).await?;
        Ok(Self { stream: ws })
    }

    pub async fn get_first_revision(&mut self) -> Result<u32, StreckenInfoError> {
        self.stream
            .send(Message::text("{\"type\":\"HANDSHAKE\",\"revision\":null}"))
            .await?;
        let msg = self
            .stream
            .next()
            .await
            .ok_or(StreckenInfoError::WebSocketNoRevisionError)??;
        let json: FirstRevisionJson = serde_json::from_slice(&msg.into_data())?;
        Ok(json.revision)
    }

    pub async fn wait_for_new_revision_filtered(
        &mut self,
        only_new_disruptions: bool,
    ) -> Result<u32, StreckenInfoError> {
        while let Some(msg) = self.stream.next().await {
            let text = msg?.into_text()?;
            if !text.starts_with('{') {
                continue;
            }
            let json: UpdateJson = serde_json::from_str(&text)?;
            if let UpdateJson::NewRevision { revision } = json {
                if only_new_disruptions && revision.disruptions.is_empty() {
                    continue;
                }
                return Ok(revision.number);
            }
        }
        Err(StreckenInfoError::WebSocketNoRevisionError)
    }

    pub async fn wait_for_new_revision(&mut self) -> Result<u32, StreckenInfoError> {
        self.wait_for_new_revision_filtered(false).await
    }
}

pub async fn get_revision() -> Result<u32, StreckenInfoError> {
    let mut ctx = RevisionContext::connect().await?;
    ctx.get_first_revision().await
}
