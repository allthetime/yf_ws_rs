use base64::prelude::*;
use chrono::DateTime;
use chrono_tz::US;
use prost::Message as ProstMessage;
use serde_json::{json, Map, Value};
use streamz::streamer::{Streamer, TungsteniteSocket};
use tungstenite::{connect, Message, Utf8Bytes};

pub mod yaticker {
    include!(concat!(env!("OUT_DIR"), "/yaticker.rs"));
}
use yaticker::Yaticker;

const YAHOO_FINANCE_WEBSOCKET_URL: &str = "wss://streamer.finance.yahoo.com/";
// const TICKER_SYMBOLS: [&str; 1] = ["HIVE.V"];
const TICKER_SYMBOLS: [&str; 1] = ["BTC-USD"];

fn build_subscription_json() -> Value {
    let mut map: Map<String, Value> = Map::new();
    let ticker_symbols_json: Value = json!(TICKER_SYMBOLS);
    map.insert("subscribe".to_string(), ticker_symbols_json);
    Value::Object(map)
}

fn subscribe_to_tickers(socket: &mut TungsteniteSocket) {
    let subscription_json = build_subscription_json();
    let msg = Message::Text(subscription_json.to_string().into());
    socket.send(msg).unwrap();
}

fn connect_socket(uri: &str) -> TungsteniteSocket {
    let (socket, response) = connect(uri).expect("Connection Failure");
    dbg!(response);
    socket
}

fn get_datetime(timestamp_ms: i64, timezone: &chrono_tz::Tz) -> DateTime<chrono_tz::Tz> {
    let timestamp: i64 = timestamp_ms / 1000;
    let dt: DateTime<chrono::Utc> =
        DateTime::from_timestamp(timestamp, 0).expect("Invalid timestamp");
    dt.with_timezone(timezone)
}

fn print_format_datetime(dt: DateTime<chrono_tz::Tz>) -> String {
    dt.format("%Y-%m-%d %H:%M:%S").to_string()
}

fn get_decodable_btyes(txt: Utf8Bytes, buffer: &mut [u8; 2048]) -> usize {
    let bytes = txt.as_bytes();
    let decoded = BASE64_STANDARD.decode(bytes).unwrap();
    buffer[..decoded.len()].copy_from_slice(&decoded);
    decoded.len()
}

fn handle_message(socket: &mut TungsteniteSocket, msg: Message, buffer: &mut [u8; 2048]) {
    match msg {
        Message::Text(txt) => {
            let size = get_decodable_btyes(txt, buffer);
            match Yaticker::decode(&buffer[..size]) {
                Ok(yaticker) => {
                    let timestamp_ms: i64 = yaticker.time;
                    let dt = get_datetime(timestamp_ms, &US::Pacific);
                    let formatted = print_format_datetime(dt);
                    dbg!(formatted);
                    dbg!(&yaticker);
                }
                Err(e) => {
                    dbg!(e);
                }
            }
        }
        Message::Ping(ping) => {
            /*
                Send Keep Alive message
            */
            let msg = Message::Pong(ping);
            socket.send(msg).unwrap();
        }
        Message::Close(close) => {
            dbg!(close);
            std::process::exit(0);
        }
        _ => (),
    }
}

fn main() {
    let mut socket = connect_socket(YAHOO_FINANCE_WEBSOCKET_URL);
    subscribe_to_tickers(&mut socket);
    let mut stream = Streamer::new(socket, handle_message);
    stream.listen();
}
