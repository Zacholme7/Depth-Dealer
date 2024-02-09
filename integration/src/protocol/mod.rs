use crate::error::SocketError;
use futures::Stream;
use serde::de::DeserializeOwned;

pub mod websocket;

// The stream parser part of the exchange stream
// anything that implements this trait will take in messange and praseit into a result
pub trait StreamParser {
    type Stream: Stream;
    type Message;
    type Error;

    fn parse<Output>(
        input: Result<Self::Message, Self::Error>,
    ) -> Option<Result<Output, SocketError>>
    where
        Output: DeserializeOwned;
}
