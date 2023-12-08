use integration::{
    error::SocketError,
    protocol::websocket::{WebSocket, WebSocketParser, WsMessage},
    ExchangeStream, Transformer,
};
use futures::{SinkExt, StreamExt};
use serde::{de, Deserialize};
use serde_json::json;
use std::str::FromStr;
use tokio_tungstenite::connect_async;
use tracing::debug;
// Convenient type alias for an `ExchangeStream` utilising a tungstenite `WebSocket`
type ExchangeWsStream<Exchange> = ExchangeStream<WebSocketParser, WebSocket, Exchange>;

type VolumeSum = f64;

#[derive(Deserialize, Debug)]
struct TradeData {
    coin: String,
    side: String,
    px: String,
    #[serde(rename = "sz")]
    quantity: f64,
    time: u64,
    hash: String,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
enum BinanceMessage {
    SubResponse {
        result: Option<Vec<String>>,
        id: u32,
    },
    Trades {
        channel: String,
        data: Vec<TradeData>,
    },
}


// transformer that contains state
struct StatefulTransformer {
    sum_of_volume: VolumeSum,
}

impl Transformer for StatefulTransformer {
    type Error = SocketError; // the type of error
    type Input = BinanceMessage; // the messages that we will be receiving
    type Output = VolumeSum; // the output of the transformation
    type OutputIter = Vec<Result<Self::Output, Self::Error>>;

    // out transformation function
    fn transform(&mut self, input: Self::Input) -> Self::OutputIter {
        match input {
            BinanceMessage::SubResponse { result, id } => {
                debug!("Received SubResponse for {}: {:?}", id, result);
            }
            BinanceMessage::Trades { data, .. } => {
                // add volume to the internal state
                for v in data.iter() {
                    self.sum_of_volume += v.quantity;
                }
            }
        }
        vec![Ok(self.sum_of_volume)]
    }
}

/// See Barter-Data for a comprehensive real-life example, as well as code you can use out of the
/// box to collect real-time public market data from many exchanges.
#[tokio::main]
async fn main() {
    // Establish Sink/Stream communication with desired WebSocket server
    let mut binance_conn = connect_async("wss://api.hyperliquid.xyz/ws")
        .await
        .map(|(ws_conn, _)| ws_conn)
        .expect("failed to connect");

    // Send something over the socket (eg/ Binance trades subscription)
    binance_conn
        .send(WsMessage::Text(
            json!({ "method": "subscribe", "subscription": { "type": "trades", "coin": "BTC" } }).to_string(),
        ))
        .await
        .expect("failed to send WsMessage over socket");

    // Instantiate some arbitrary Transformer to apply to data parsed from the WebSocket protocol
    let transformer = StatefulTransformer { sum_of_volume: 0.0 };

    // ExchangeWsStream includes pre-defined WebSocket Sink/Stream & WebSocket StreamParser
    let mut ws_stream = ExchangeWsStream::new(binance_conn, transformer);

    // Receive a stream of your desired Output data model from the ExchangeStream
    while let Some(volume_result) = ws_stream.next().await {
        match volume_result {
            Ok(cumulative_volume) => {
                // Do something with your data
                println!("{cumulative_volume:?}");
            }
            Err(error) => {
                // React to any errors produced by the internal transformation
                eprintln!("{error}")
            }
        }
    }
}

/// Deserialize a `String` as the desired type.
fn de_str<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: de::Deserializer<'de>,
    T: FromStr,
    T::Err: std::fmt::Display,
{
    let data: String = Deserialize::deserialize(deserializer)?;
    data.parse::<T>().map_err(de::Error::custom)
}