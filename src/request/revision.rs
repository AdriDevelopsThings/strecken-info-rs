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
use tokio_tungstenite::{
    connect_async,
    tungstenite::{error::ProtocolError, Error, Message},
    MaybeTlsStream, WebSocketStream,
};

use crate::error::StreckenInfoError;

const WEBSOCKET_PATH: &str = "wss://strecken-info.de/api/websocket";

pub struct RevisionContext {
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    old_revision: Option<u32>,
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

/// returns `true` if `err` is
/// - Error::ProtocolError::ResetWithoutClosingHandshake
/// - Error::ConnectionClosed
fn revision_error_should_retry(err: &Error) -> bool {
    if let Error::Protocol(prtctl_err) = err {
        return matches!(prtctl_err, ProtocolError::ResetWithoutClosingHandshake);
    }

    matches!(err, Error::ConnectionClosed)
}

impl RevisionContext {
    pub async fn connect() -> Result<Self, StreckenInfoError> {
        let (ws, _) = connect_async(WEBSOCKET_PATH).await?;
        Ok(Self {
            stream: ws,
            old_revision: None,
        })
    }

    /// close the open stream and reopen it (doing the handshake with `get_first_revision` is mandatory)
    async fn reconnect(&mut self) -> Result<(), StreckenInfoError> {
        // ignore close result (we want to force the close)
        let _ = self.stream.close(None).await;
        *self = Self::connect().await?;
        Ok(())
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
        self.old_revision = Some(json.revision);
        Ok(json.revision)
    }

    pub async fn wait_for_new_revision_filtered(
        &mut self,
        only_new_disruptions: bool,
    ) -> Result<u32, StreckenInfoError> {
        if self.old_revision.is_none() {
            return self.get_first_revision().await;
        }

        // just one retry is allowed
        let mut retry = true;

        while let Some(msg) = self.stream.next().await {
            if let Err(err) = msg {
                if revision_error_should_retry(&err) && retry {
                    let old_revision = self.old_revision;
                    self.reconnect().await?;
                    let revision = self.get_first_revision().await?;
                    if old_revision != Some(revision) {
                        return Ok(revision);
                    }

                    retry = false;
                    continue;
                } else {
                    return Err(StreckenInfoError::WebsocketError(err));
                }
            }

            let text = msg?.into_text()?;
            retry = true;
            // no json (e.g. a 'PING')
            if !text.starts_with('{') {
                continue;
            }

            let json: UpdateJson = serde_json::from_str(&text)?;
            if let UpdateJson::NewRevision { revision } = json {
                self.old_revision = Some(revision.number);
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
