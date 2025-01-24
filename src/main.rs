use std::env;
use std::path::PathBuf;

// use openssl::ssl::SslStream;
use prost::Message as ProstMessage;
use serde_json::{json, Map, Value};
use tungstenite::http::{response, Uri};
use tungstenite::{connect, Message};

use prost_reflect::{DescriptorPool, DynamicMessage, Value as ProstValue};

const YAHOO_FINANCE_WEBSOCKET_URL: &str = "wss://streamer.finance.yahoo.com/";
const TICKER_SYMBOLS: [&str; 1] = ["AAPL"];

pub mod yaticker {
    include!(concat!(env!("OUT_DIR"), "/yaticker.rs"));
}

use yaticker::Yaticker;

fn build_subscription_object(ticker_symbols: Vec<&str>) -> Value {
    let mut map = Map::new();
    let ticker_symbols_json = json!(ticker_symbols);
    map.insert("subscribe".to_string(), ticker_symbols_json);
    Value::Object(map)
}

fn main() {
    let uri: Uri = YAHOO_FINANCE_WEBSOCKET_URL.parse().unwrap();

    let (mut socket, response) = connect(uri).expect("Failed to connect");

    dbg!(response);

    let subscription_json = build_subscription_object(TICKER_SYMBOLS.to_vec());
    let msg = Message::Text(subscription_json.to_string().into());

    dbg!(&msg);

    dbg!(Yaticker::default());

    socket.send(msg).unwrap();

    let file_descriptor_path =
        PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR environment variable not set"))
            .join("file_descriptor_set.bin");

    let file_descriptor_bytes =
        std::fs::read(file_descriptor_path).expect("Failed to read file descriptor");
    let pool = DescriptorPool::decode(file_descriptor_bytes.as_ref()).unwrap();
    let message_descriptor = pool.get_message_by_name("yaticker.yaticker").unwrap();

    loop {
        let msg = socket.read().unwrap();
        match msg {
            Message::Text(txt) => {
                println!("Received TXT: {}", txt);

                let dynamic_message =
                    DynamicMessage::decode(message_descriptor.clone(), txt.as_bytes());

                match dynamic_message {
                    Ok(dynamic_message) => {
                        println!("Received DynamicMessage: {:?}", dynamic_message);
                    }
                    Err(e) => {
                        print!("Error decoding DynamicMessage: ");
                        dbg!(e);
                    }
                }

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
            Message::Binary(bin) => {
                println!("Received BIN: {:?}", bin);
            }
            Message::Ping(ping) => {
                println!("Received: Ping {:?}", ping);
            }
            Message::Pong(pong) => {
                println!("Received: Pong {:?}", pong);
            }
            Message::Close(close) => {
                println!("Received: Close {:?}", close);
                break;
            }
            Message::Frame(frame) => todo!(),
        }
    }
}
