use std::io::Read;

use base64::prelude::*;
use prost::Message as ProstMessage;
use serde_json::{json, Map, Value};
use streamz::streamer;
use tungstenite::http::Uri;
use tungstenite::{connect, Message};

pub mod yaticker {
    include!(concat!(env!("OUT_DIR"), "/yaticker.rs"));
}
use yaticker::Yaticker;

const YAHOO_FINANCE_WEBSOCKET_URL: &str = "wss://streamer.finance.yahoo.com/";
const TICKER_SYMBOLS: [&str; 1] = ["BTC-CAD"];

struct Heart {
    timestamp: std::time::Instant,
}

impl Heart {
    fn new() -> Self {
        Self {
            timestamp: std::time::Instant::now(),
        }
    }
    fn beat(&mut self, to_do: Box<dyn Fn(&std::time::Instant) -> std::time::Instant>) {
        self.timestamp = (to_do)(&self.timestamp);
    }
}

type TungsteniteSocket =
    tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>;


fn build_subscription_json() -> Value {
    let mut map = Map::new();
    let ticker_symbols_json = json!(TICKER_SYMBOLS);
    map.insert("subscribe".to_string(), ticker_symbols_json);
    Value::Object(map)
}

fn subscribe_to_tickers(socket: &mut TungsteniteSocket) {
    let subscription_json = build_subscription_json();
    let msg = Message::Text(subscription_json.to_string().into());
    socket.send(msg).unwrap();
}

fn connect_socket(uri: Uri) -> TungsteniteSocket {
    let (socket, response) = connect(uri).expect("Connection Failure");
    print!("Connected to Yahoo Finance WebSocket: ");
    dbg!(response);
    socket
}

fn main() {
    let uri: Uri = YAHOO_FINANCE_WEBSOCKET_URL.parse().unwrap();
    let mut socket: TungsteniteSocket = connect_socket(uri);
    subscribe_to_tickers(&mut socket);
    let mut heart: Heart = Heart::new();

    let streamer = streamer::Streamer {
        stream: socket,
        transformer: Box::new(|msg| {
            print!("Received Message: ");
            dbg!(msg);
        }),
    }

    // poll socket
    // TODO: make this async

    loop {
        let msg = socket.read().unwrap();

        match msg {
            Message::Text(txt) => {
                let decoded = BASE64_STANDARD.decode(txt.as_bytes()).unwrap();
                match Yaticker::decode(decoded.as_slice()) {
                    Ok(yaticker) => {
                        print!("Received Yaticker: ");
                        dbg!(yaticker);
                    }
                    Err(e) => {
                        print!("Error decoding Yaticker: ");
                        dbg!(e);
                    }
                }
            }
            Message::Binary(bin) => {
                println!("Received Binary message: {:?}", bin);
            }
            Message::Ping(ping) => {
                heart.beat(Box::new(|last_heartbeat| {
                    let now = std::time::Instant::now();
                    let time_since_last_heartbeat = now.duration_since(*last_heartbeat);
                    dbg!(time_since_last_heartbeat);
                    now
                }));

                /*
                    Send Keep Alive message
                */
                let msg = Message::Pong(ping);
                socket.send(msg).unwrap();
            }
            Message::Pong(pong) => {
                println!("Received Pong message: {:?}", pong);
            }
            Message::Close(close) => {
                println!("Received Close message: {:?}", close);
            }
            Message::Frame(frame) => {
                println!("Received Frame message: {:?}", frame);
            }
        }
    }
}
