use chrono::{DateTime, Utc};
use crate::integration::model::{
    model::Exchange,
    instrument::Instrument,
}


//#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Deserialize, Serialize)]
pub struct MarketEvent<T> {
    pub exchange_time: DateTime<Utc>,
    pub received_time: DateTime<Utc>,
    pub exchange: Exchange, // this was defined in integration
    pub instrument: Instrument, // this was defined in integration
    pub kind: T,
}
