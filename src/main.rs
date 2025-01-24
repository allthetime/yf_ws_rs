// use openssl::ssl::SslStream;
use prost::Message as ProstMessage;
use serde_json::{json, Map, Value};
use tungstenite::http::Uri;
use tungstenite::{connect, Message};

pub mod yaticker;

use quick_protobuf::{BytesReader, MessageRead};
use yaticker::Yaticker;

const YAHOO_FINANCE_WEBSOCKET_URL: &str = "wss://streamer.finance.yahoo.com/";
const TICKER_SYMBOLS: [&str; 1] = ["AAPL"];

// pub mod yaticker {
//     include!(concat!(env!("OUT_DIR"), "/yaticker.rs"));
// }

// use yaticker::Yaticker;

fn build_subscription_object(ticker_symbols: Vec<&str>) -> Value {
    let mut map = Map::new();
    let ticker_symbols_json = json!(ticker_symbols);
    map.insert("subscribe".to_string(), ticker_symbols_json);
    Value::Object(map)
}

fn main() {
    let uri: Uri = YAHOO_FINANCE_WEBSOCKET_URL.parse().unwrap();

    let (mut socket, response) = connect(uri).expect("Failed to connect");
    let subscription_json = build_subscription_object(TICKER_SYMBOLS.to_vec());
    let msg = Message::Text(subscription_json.to_string().into());

    socket.send(msg).unwrap();

    loop {
        let msg = socket.read().unwrap();

        if let Message::Text(txt) = msg {
            dbg!(&txt);
            dbg!(txt.as_bytes());

            let mut reader = BytesReader::from_bytes(txt.as_bytes());
            let yaticker = Yaticker::from_reader(&mut reader, txt.as_bytes()).expect("CANT READ");

            // match Yaticker::decode(txt.as_bytes()) {
            //     Ok(yaticker) => {
            //         println!("Received Yaticker: {:?}", yaticker);
            //     }
            //     Err(e) => {
            //         print!("Error decoding Yaticker: ");
            //         dbg!(e);
            //     }
            // }
        }
    }
}
