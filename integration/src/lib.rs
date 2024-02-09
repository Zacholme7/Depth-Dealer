
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


// determien if the internal statis is satisfactor to fulfill some use case defined by the
// implementor
pub trait Validator {
    fn validate(self) -> Result<Self, SocketError>
    where
        Self: Sized;
}


// transform any input into an iterator
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
    InnerStream: Stream,
    Protocol: StreamParser,
    StreamTransformer: Transformer,
{
    #[pin]
    pub stream: InnerStream, // futures stream, trait that is a stream of values produced async,
                             // needs a poll next
    pub transformer: StreamTransformer, // the transformer defined above, transform intput into
                                        // terator
    pub buffer: VecDeque<Result<StreamTransformer::Output, StreamTransformer::Error>>,
    pub protocol_marker: PhantomData<Protocol>, // the stream parser, will parse the stream
}

impl<Protocol, InnerStream, StreamTransformer> Stream
    for ExchangeStream<Protocol, InnerStream, StreamTransformer>
where
    Protocol: StreamParser,
    InnerStream: Stream<Item = Result<Protocol::Message, Protocol::Error>> + Unpin,
    StreamTransformer: Transformer,
    StreamTransformer::Error: From<SocketError>,
{
    type Item = Result<StreamTransformer::Output, StreamTransformer::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        loop {
            // Flush Self::Item buffer if it is not currently empty
            if let Some(output) = self.buffer.pop_front() {
                return Poll::Ready(Some(output));
            }

            // Poll inner `Stream` for next the next input protocol message
            let input = match self.as_mut().project().stream.poll_next(cx) {
                Poll::Ready(Some(input)) => input,
                Poll::Ready(None) => return Poll::Ready(None),
                Poll::Pending => return Poll::Pending,
            };

            // Parse input protocol message into `ExchangeMessage`
            let exchange_message = match Protocol::parse::<StreamTransformer::Input>(input) {
                // `StreamParser` successfully deserialised `ExchangeMessage`
                Some(Ok(exchange_message)) => exchange_message,

                // If `StreamParser` returns an Err pass it downstream
                Some(Err(err)) => return Poll::Ready(Some(Err(err.into()))),

                // If `StreamParser` returns None it's a safe-to-skip message
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

impl<Protocol, InnerStream, StreamTransformer>
    ExchangeStream<Protocol, InnerStream, StreamTransformer>
where
    Protocol: StreamParser,
    InnerStream: Stream,
    StreamTransformer: Transformer,
{
    pub fn new(stream: InnerStream, transformer: StreamTransformer) -> Self {
        Self {
            stream,
            transformer,
            buffer: VecDeque::with_capacity(6),
            protocol_marker: PhantomData::default(),
        }
    }
}


