use crate::{error::SocketError, protocol::StreamParser};
use futures::Stream;
use pin_project::pin_project;
use serde::Deserialize;
use std::{
    collections::VecDeque,
    fmt::Debug,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

pub mod error;
pub mod model;
pub mod protocol;

pub trait Validator {
    fn validate(self) -> Result<Self, SocketError>
    where
        Self: Sized;
}

pub trait Transformer {
    type Error;
    type Input: for<'de> Deserialize<'de>;
    type Output;
    type OutputIter: IntoIterator<Item = Result<Self::Output, Self::Error>>;
    fn transform(&mut self, input: Self::Input) -> Self::OutputIter;
}

// Exchange stream struct
#[derive(Debug)]
#[pin_project]

pub struct ExchangeStream<Protocol, InnerStream, StreamTransformer>
where
    Protocol: StreamParser,
    InnerStream: Stream,
    StreamTransformer: Transformer,
{
    #[pin]
    pub stream: InnerStream,
    pub transformer: StreamTransformer,
    pub buffer: VecDeque<Result<StreamTransformer::Output, StreamTransformer::Error>>,
    pub protocol_market: PhantomData<Protocol>,
}

impl<Protocol, InnerStream, StreamTransformer> Stream
    for ExchangeStream<Protocol, InnerStream, StreamTransformer>
where
    Protocol: StreamParser,
    InnerStream: Stream<Item = Result<Protocol::Message, Protocol::Error>> + Unpin,
    StreamTransformer: Transformer,
    StreamTransformer::Error: From<SocketError>
{
    type Item = Result<StreamTransformer::Output, StreamTransformer::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {

        loop {
            if let Some(output) = self.buffer.pop_front() {
                return Poll::Ready(Some(output));
            }

            let input = match self.as_mut().project().stream.poll_next(cx) {
                Poll::Ready(Some(Input)) => input,
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending => return Poll::Pending,
            };


            let exchange_message = match Protocol::parse<StreamTransformer::Input>(input) {
                Some(Ok(exchange_message)) => exchange_message,
                Some(Err(err)) => return Poll::Ready(Some(Err(err.into()))),
                None => continue,
            };







        }

    }
}
