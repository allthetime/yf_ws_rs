use std::io::{Read, Write};

use tungstenite::Message;

pub struct Streamer<T: Read + Write> {
    pub stream: T,
    pub transformer: Box<dyn Fn(Message)>,
}
