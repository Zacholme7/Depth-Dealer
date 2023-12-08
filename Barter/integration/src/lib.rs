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
pub mod model;
pub mod protocol;
pub mod error;



/// [`Validator`]s are capable of determining if their internal state is satisfactory to fulfill
/// some use case defined by the implementor.
pub trait Validator {
    /// Chech if `Self` if valid for some use case
    fn validate(self) -> Result<Self, SocketError>
    where
        Self: Sized;
}

/// [`Transformer`]s are capable of transforming any `Input` into an iterator of
/// `Result<Self::Output, Self::Error>`s.
pub trait Transformer {
    type Error; 
    type Input: for<'de> Deserialize<'de>;
    type Output;
    type OutputIter: IntoIterator<Item = Result<Self::Output, Self::Error>>;
    fn transform(&mut self, input: Self::Input) -> Self::OutputIter;
}

/// An [`ExchangeStream`] is a communication protocol agnostic [`Stream`]. It polls protocol
/// messages from the inner [`Stream`], and transforms them into the desired output data structure.
#[derive(Debug)]
#[pin_project]
pub struct ExchangeStream<Protocol, InnerStream, StreamTransformer>
where
    Protocol: StreamParser,
    InnerStream: Stream,
    StreamTransformer: Transformer
{
    #[pin]
    pub stream: InnerStream, // the stream that will recieve messages
    pub transformer: StreamTransformer, // transforms the input stream into iterator
    pub buffer: VecDeque<Result<StreamTransformer::Output, StreamTransformer::Error>>,
    pub protocol_market: PhantomData<Protocol>,
}

impl<Protocol, InnerStream, StreamTransformer> Stream for ExchangeStream<Protocol, InnerStream, StreamTransformer>
where
    Protocol: StreamParser,
    InnerStream: Stream<Item = Result<Protocol::Message, Protocol::Error>> + Unpin,
    StreamTransformer: Transformer,
    StreamTransformer::Error: From<SocketError>
{
    // Result time of the output and the error
    type Item = Result<StreamTransformer::Output, StreamTransformer::Error>;

    // need to implement poll next for the stream
    // self is a pin, so we are ensuring that it will not be moved/the memory will not be moved
    // this is because it is a self referenctial struct
    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {

            // we have an element that we are ready to process
            // flush Self::Item bugger if it is not currently empty
            if let Some(output) = self.buffer.pop_front() {
                // signal that we have something ready
                return Poll::Ready(Some(output));
            }

            // poll inner stream for next the next input protocol mesage
            // this is getting the next message
            // project is something to do with pin_project
            let input = match self.as_mut().project().stream.poll_next(cx) {
                Poll::Ready(Some(input)) => input,
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending => return Poll::Pending
            };

            // Parse input protocol message into ExchangeMessage
            let exchange_message = match Protocol::parse::<StreamTransformer::Input>(input) {
                // Stream parser successfully deserialized ExchangeMesasge
                Some(Ok(exchange_message)) => exchange_message,

                // if StreamParser returns an err pass it downstream
                Some(Err(err)) => return Poll::Ready(Some(Err(err.into()))),

                // if streamparser returns none, it is safe to skip
                None => continue,
            };


            // Transform `ExchangeMessage` into `Transformer::OutputIter`
            // ie/ IntoIterator<Item = Result<Output, SocketError>>
            self.transformer
                .transform(exchange_message)
                .into_iter()
                .for_each(
                    |output_result: Result<StreamTransformer::Output, StreamTransformer::Error>| {
                        self.buffer.push_back(output_result)
                    },
                );
        }
    }
}



impl<Protocol, InnerStream, StreamTransformer> ExchangeStream<Protocol, InnerStream, StreamTransformer>
where
    Protocol: StreamParser,
    InnerStream: Stream,
    StreamTransformer: Transformer
{
    // implement our constructor
    pub fn new(stream: InnerStream, transformer: StreamTransformer) -> Self {
        Self {
            stream,
            transformer,
            buffer: VecDeque::with_capacity(6),
            protocol_market: PhantomData::default(),
        }
    }
}






































