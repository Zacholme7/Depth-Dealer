//use crate::{error::SocketError, protocol::StreamParser};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::fmt::Debug;
use tokio::net::TcpStream;
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        client::IntoClientRequest,
        error::ProtocolError,
        protocol::{frame::Frame, CloseFrame},
    },
    MaybeTlsStream,
};

/// Convenient type alias for a tungstenite `WebSocketStream`
pub type WebSocket = tokio_tungstenite::WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Convenient type alias for the `Sink` half of a tungstenite [`WebSocket`]
pub type WsSink = futures::stream::SplitSink<WebSocket, WsMessage>;

/// Convenient type alias for the `Stream` half of a tungstenite [`WebSocket`].
pub type WsStream = futures::stream::SplitStream<WebSocket>;

/// Communicative type alias for a tungstenite [`WebSocket`] `Message`
pub type WsMessage = tokio_tungstenite::tungstenite::Message;

/// Communicative type alias for a tungstenite [`WebSocket`] `Error`.
pub type WsError = tokio_tungstenite::tungstenite::Error;

/// Default [`StreamParser`] implementation for a [`WebSocket`].
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct WebSocketParser;

