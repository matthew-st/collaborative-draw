use std::{collections::HashMap, hash::Hash, net::TcpStream};
use serde::{Serialize, Deserialize};

pub mod captcha {
    use serde::{Serialize, Deserialize};
    #[derive(Serialize, Deserialize)]
    pub struct Request {
        pub secret: String,
        pub response: String,
    }

    #[derive(Serialize, Deserialize)]
    pub struct Response {
        pub success: bool,
        pub score: f32,
        pub action: String,
        pub challenge_ts: String,
        pub hostname: String,
        pub error_codes: Option<Vec<String>>,
    }
}



pub struct Client {
    pub socket: TcpStream,
}

pub type Colour = u8;

#[derive(Serialize, Deserialize, Debug)]

pub struct Packet {
    pub t: String,
    pub d: Packets
}

#[derive(Serialize, Deserialize, Debug)]
pub enum Packets {
    Init(BoardInit),
    Paint(BoardPaint),
    InitResponse(BoardInitResponse),
    Update(BoardUpdate),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoardInit {
    pub captcha: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoardInitResponse {
    pub board: Board,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoardPaint {
    pub x: u16,
    pub y: u16,
    pub colour: Colour,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BoardUpdate {
    pub x: u16,
    pub y: u16,
    pub colour: Colour
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Board (HashMap<Key, u8>, u16);

#[derive(Eq, PartialEq, Hash, Clone, Debug, Serialize, Deserialize)]
pub struct Key(pub u16, pub u16);

impl Board {
    pub fn new(size: u16) -> Board {
        Board(HashMap::new(), size)
    }

    pub fn set(&mut self, x: u16, y: u16, value: u8) {
        if x > self.1 || y > self.1 {
            return;
        }
        self.0.insert(Key(x,y), value);
    }

    pub fn get(&self, x: u16, y: u16) -> Option<u8> {
        self.0.get(&Key(x,y)).cloned()
    }

    pub fn get_board(&self) -> Board {
        Board(self.0.clone(), self.1.clone())
    }
}