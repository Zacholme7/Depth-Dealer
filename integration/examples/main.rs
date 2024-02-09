use futures::{SinkExt, StreamExt};
use integration::{
    error::SocketError,
    protocol::websocket::{WebSocket, WebSocketParser, WsMessage},
    ExchangeStream, Transformer,
};
use serde::{de, Deserialize};
use serde_json::json;
use std::str::FromStr;
use tokio_tungstenite::connect_async;
use tracing::debug;

/*
type VolumeSum = f64;

#[derive(Deserialize)]
#[serde(untagged, rename_all = "camelCase")]
enum BinanceMessage {
    SubResponse {
        result: Option<Vec<String>>,
        id: u32,
    },
    Trade {
        #[serde(rename = "q", deserialize_with = "de_str")]
        quantity: f64,
    },
}

struct StatefulTransformer {
    sum_of_volume: VolumeSum
}


impl Transformer for StatefulTransformer {
    type Error = SocketError; // socket error is what it can have
    type Input = BinanceMessage; // will be transforming binance message
    type Output = VolumeSum; // output the volume sum
    type OutputIter = Vec<Result<Self::Output, Self::Error>>;

    // input is the binance message and it is outputting an iterator over the vector
    fn transform(&mut self, input: Self::Input) -> Self::OutputIter {
        match input {
            BinanceMessage::SubResponse { result, id } => {
                println!("subscribed");
            }
            BinanceMessage::Trade { quantity, .. } => {
                self.sum_of_volume += quantity;
            }
        };
        vec![Ok(self.sum_of_volume)]
    }

}
*/


#[tokio::main]
async fn main() {

    // get the websocket
    let mut binance_conn = connect_async("wss://fstream.binance.com/ws/")
        .await
        .map(|(ws_conn, _)| ws_conn)
        .expect("failed to connect");

    // send a subscription
    binance_conn
        .send(WsMessage::Text(
            json!({"method": "SUBSCRIBE","params": ["btcusdt@aggTrade"],"id": 1}).to_string(),
        ))
        .await
        .expect("failed to send WsMessage over socket");





    // construct the transformer
    //
    //let transformer = StatefulTransformer {sum_of_volume: 0.0};


    println!("hello")

}


/*
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
*/
