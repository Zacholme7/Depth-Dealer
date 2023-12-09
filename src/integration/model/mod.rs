use crate::integration::model::instrument::{kind::InstrumentKind, symbol::Symbol, Instrument};
use serde::{Deserialize, Deserializer, Serialize};
use std::{
    borrow::Cow,
    fmt::{Debug, Display, Formatter},
};


/// [`Instrument`] related data structures.
///
/// eg/ `Instrument`, `InstrumentKind`, `OptionContract`, `Symbol`, etc.
pub mod instrument;

/// Represents a unique combination of an [`Exchange`] & an [`Instrument`].
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub struct Market {
    pub exchange: Exchange,
    #[serde(flatten)]
    pub instrument: Instrument 
}

// can create market instance from any types E and I where
// E can be converted into an Exchange and I can be converted into 
// an instrument
impl<E, I> From<(E, I)> for Market
where
    E: Into<Exchange>,
    I: Into<Instrument>,
{
    fn from((exchange, instrument): (E, I)) -> Self {
        Self::new(exchange, instrument)
    }
}

impl<E, S> From<(E, S, S, InstrumentKind)> for Market
where
    E: Into<Exchange>,
    S: Into<Symbol>,
{
    fn from((exchange, base, quote, instrument_kind): (E, S, S, InstrumentKind)) -> Self {
        Self::new(exchange, (base, quote, instrument_kind))
    }
}


impl Market {
        /// Constructs a new [`Market`] using the provided [`Exchange`] & [`Instrument`].
        pub fn new<E, I>(exchange: E, instrument: I) -> Self
        where
            E: Into<Exchange>,
            I: Into<Instrument>
        {
            Self {
                exchange: exchange.into(),
                instrument: instrument.into(),
            }
        }
}

/// Barter new type representing a unique `String` identifier for a [`Market`], where a [`Market`]
/// represents an [`Instrument`] being traded on an [`Exchange`].
///
/// eg/ binance_btc_usdt_spot
/// eg/ ftx_btc_usdt_future_perpetual
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize)]
pub struct MarketId(pub String);

impl<'a, M> From<M> for MarketId
where
    M: Into<&'a Market>
{
    fn from(market: M) -> Self{
        let market = market.into();
        Self::new(&market.exchange, &market.instrument)
    }
}

// implement the debug trait
impl Debug for MarketId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// implement the display trait
impl Display for MarketId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// make sure we can deserliaze this
impl<'de> Deserialize<'de> for MarketId {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        String::deserialize(deserializer).map(MarketId)
    }
}

impl MarketId {
    /// Construct a unique `String` [`MarketId`] identifier for a [`Market`], where a [`Market`]
    /// represents an [`Instrument`] being traded on an [`Exchange`].
    pub fn new(exchange: &Exchange, instrument: &Instrument) -> Self {
        Self(
            format!(
                "{}_{}_{}_{}",
                exchange, instrument.base, instrument.quote, instrument.kind
            )
            .to_lowercase(),
        )
    }
}



/// Barter representation of an [`Exchange`]'s name.
///
/// eg/ Exchange("binance_spot"), Exchange("bitfinex"), Exchange("gateio_spot"), etc.
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
pub struct Exchange(Cow<'static, str>);

// implemeation of the from trait for the exchange type
// means that exchange can be created from some other type T
// From will work from any type E as long as E satifies contraint in the where clause
// Where clause means that E can be converted into Cow<'static, str>
// calls into() on the exchange which converts it into a Cow<static, str> since e implements this then consturcts the exchange from it

impl<E> From<E> for Exchange
where
    E: Into<Cow<'static, str>>,
{
    fn from(exchange: E) -> Self {
        Exchange(exchange.into())
    }
}

// implement the debug trait
impl Debug for Exchange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// implement the display trait
impl Display for Exchange {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}



/// New type representing a unique `String` identifier for a stream that has been subscribed to.
/// This is used to identify data structures received over the socket.
///
/// For example, `Barter-Data` uses this identifier to associate received data structures from the
/// exchange with the original `Barter-Data` `Subscription` that was actioned over the socket.
///
/// Note: Each exchange will require the use of different `String` identifiers depending on the
/// data structures they send.
///
/// eg/ [`SubscriptionId`] of an `FtxTrade` is "{BASE}/{QUOTE}" (ie/ market).
/// eg/ [`SubscriptionId`] of a `BinanceTrade` is "{base}{symbol}@trade" (ie/ channel).
#[derive(Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Deserialize, Serialize)]
pub struct SubscriptionId(pub String);

// implement debug for SubscriptionId
impl Debug for SubscriptionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// implement display for SubscriptionId
impl Display for SubscriptionId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl AsRef<str> for SubscriptionId {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

impl<S> From<S> for SubscriptionId
where
    S: Into<String>
{
    fn from(input: S) -> Self {
        Self(input.into())
    }
}


/// [`Side`] of a trade or position - Buy or Sell.
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug, Deserialize, Serialize)]
pub enum Side {
    #[serde(alias = "buy", alias = "BUY", alias = "b")]
    Buy,
    #[serde(alias = "sell", alias = "SELL", alias = "s")]
    Sell,
}

impl Display for Side {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Side::Buy => "buy",
            Side::Sell => "sell"
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*; // make sure we have everything from this file
    use crate::model::instrument::{kind::InstrumentKind, Instrument};
    use serde::de::Error;

    #[test]
    fn test_de_market() {
        struct TestCase {
            input: &'static str,
            expected: Result<Market, serde_json::Error>,
        }

        let cases = vec![
            TestCase {
                // TC0: Valid Binance btc_usd Spot Market
                input: r##"{ "exchange": "binance", "base": "btc", "quote": "usd", "instrument_kind": "spot" }"##,
                expected: Ok(Market {
                    exchange: Exchange::from("binance"),
                    instrument: Instrument::from(("btc", "usd", InstrumentKind::Spot)),
                }),
            },
            TestCase {
                // TC1: Valid Ftx btc_usd FuturePerpetual Market
                input: r##"{ "exchange": "ftx_old", "base": "btc", "quote": "usd", "instrument_kind": "perpetual" }"##,
                expected: Ok(Market {
                    exchange: Exchange::from("ftx_old"),
                    instrument: Instrument::from(("btc", "usd", InstrumentKind::Perpetual)),
                }),
            },
            TestCase {
                // TC3: Invalid Market w/ numeric exchange
                input: r##"{ "exchange": 100, "base": "btc", "quote": "usd", "instrument_kind": "perpetual" }"##,
                expected: Err(serde_json::Error::custom("")),
            },
        ];

        for (index, test) in cases.into_iter().enumerate() {
            // convert from string into Makret object
            let actual = serde_json::from_str::<Market>(test.input);

            match (actual, test.expected) {
                (Ok(actual), Ok(expected)) => {
                    assert_eq!(actual, expected, "TC{} failed", index)
                }
                (Err(_), Err(_)) => {
                    // Test passed
                }
                (actual, expected) => {
                    // Test failed
                    panic!("TC{index} failed because actual != expected. \nActual: {actual:?}\nExpected: {expected:?}\n");
                }
            }
        }

    }
}






































