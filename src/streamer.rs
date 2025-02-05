use tungstenite::Message;

pub type TungsteniteSocket =
    tungstenite::WebSocket<tungstenite::stream::MaybeTlsStream<std::net::TcpStream>>;

pub struct Streamer {
    pub socket: TungsteniteSocket,
    pub on_message: fn(&mut TungsteniteSocket, Message, &mut [u8; 2048]),
    buffer: [u8; 2048],
}

impl Streamer {
    pub fn new(
        socket: TungsteniteSocket,
        on_message: fn(&mut TungsteniteSocket, Message, &mut [u8; 2048]),
    ) -> Self {
        Self {
            socket,
            on_message,
            buffer: [0; 2048],
        }
    }

    pub fn listen(&mut self) {
        loop {
            let msg = self.socket.read().unwrap();
            (self.on_message)(&mut self.socket, msg, &mut self.buffer);
        }
    }
}

//
// TODO:
// refactor with
// https://docs.rs/fastwebsockets/latest/fastwebsockets/
// also... get binance stream as well!
// do some comparisons?
//
