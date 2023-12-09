use crate::exchange::ExchangeId;
use crate::subscription::SubKind;
use tokio::sync::mpsc;


use std::fmt::Debug;
use std::collections::HashMap;



// stream builder instance that will do the building
#[derive(Default)]
pub struct StreamBuilder<Kind>
where  
    Kind: SubKind
{
    // map the exchange id to an exchange channel which will take in the info
    pub channel: HashMap<ExchangeId, ExchangeChannel<MarketEvent<Kind::Event>>>,
    // not sure what this is, maybe that we are waiting to get back that we want to subscribe
    pub futures: Vec<SubscribeFuture>, 
}

// implement debug trait for the StreamBuilder
impl<Kind> Debug for StreamBuilder<Kind> 
where
    Kind: SubKind
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // debug struct is used when implemeneting debug for custom types
        // pass the name of the struct as a string, returns DebugStruct builder
        f.debug_struct("StreamBuilder<SubKind>")
            .field("channels", &self.channels)
            .field("num_futures", &self.futures.len())
            .finish()
    }
}

impl<Kind> StreamBuilder<Kind> 
where
    Kind: SubKind
{
    // construct a new instance of the stream builder
    pub fn new() -> Self {
        Self {
            channel: HashMap::new(),
            futures: Vec::new()
        }
    }
}


// the channel for the exchange
// sender and receiver
#[derive(Debug)]
pub struct ExchangeChannel<T> {
    tx: mpsc::UnboundedSender<T>,
    rx: mpsc::UnboundedReceiver<T>
}

impl<T> ExchangeChannel<T> {
    // constructs a new self
    pub fn new() -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        Self { tx, rx }
    }
}


































