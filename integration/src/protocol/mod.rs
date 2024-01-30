use crate::error::SocketError;
use futures::Stream;
use serde::de::DeserializeOwned;

pub mod websocket;

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
