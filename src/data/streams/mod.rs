use std::collections::HashMap; // just your standard hashmap
use tokio::sync::mpsc; // multiple producer single consumer for async applications
use self::builder::StreamBuilder;
use crate::{
    exchange::ExchangeId,
    subscription::SubKind
};

pub mod builder;


// all of the streams that we want to connect to
#[derive(Debug)]
pub struct Streams<T> {
    // we have a hashmap that contains all of the exchanges that we want to connec tto
    pub stream: HashMap<ExchangeId, mpsc::UnboundedReceiver<T>>,
}

impl<T> Streams<T> {
    // call the builder function that will return a StreamBuilder
    // this uses the builder pattern and lets us add lots of streams
    pub fn builder<Kind>() -> StreamBuilder<Kind>
    where
        Kind: SubKind,
    {
        StreamBuilder::<Kind>::new()
    }
}

