#[macro_use]
extern crate lazy_static;
mod captcha;
mod structs;
use dotenv::dotenv;
use futures::lock::Mutex;
use std::net::{TcpListener, TcpStream};
use std::sync::Arc;

lazy_static! {
    pub static ref BOARD: Arc<Mutex<structs::Board>> =
        Arc::new(Mutex::new(structs::Board::new(2000)));
    pub static ref CLIENTS: Arc<Mutex<Vec<structs::Client>>> = Arc::new(Mutex::new(Vec::new()));
}

fn main() {
    dotenv().ok();
    let listener = TcpListener::bind("0.0.0.0:8080").unwrap();
    println!("Listening on port 8080");
    loop {
        match listener.accept() {
            Ok((mut stream, address)) => {
                println!("[TCPListener] Connection from {}", address);
                match tungstenite::accept(&mut stream) {
                    Ok(_) => {
                        println!("[WebSocket] Opened onnection from {}", address);
                        tokio::spawn(async move {
                            ws_handler(stream).await;
                        });
                    }
                    Err(_) => {
                        println!("[WebSocket] Error on connection from {}", address);
                    }
                };
            }
            Err(e) => {
                println!("accept() error: {}", e);
                break;
            }
        }
    }
}

async fn ws_handler(socket: TcpStream) {
    let mut array = CLIENTS.lock().await;
    array.push(structs::Client {
        socket: socket.try_clone().unwrap(),
    });
    drop(array);
    loop {
        let stream = socket.try_clone().unwrap();
        match websocket(&stream).read_message() {
            Ok(n) => match n {
                tungstenite::Message::Text(message) => {
                    let packet = match serde_json::de::from_str(&message) {
                        Ok(p) => p,
                        Err(e) => {
                            println!("[WebSocket] Error deserializing packet: {}", e);
                            continue;
                        }
                    };
                    match packet {
                        structs::Packets::Init(p) => {
                            let captcha = captcha::validate(p.captcha).await;
                            if !captcha {
                                println!("[WebSocket] Captcha failed");
                                websocket(&stream).close(None).unwrap();
                                continue;
                            }
                            let board = BOARD.lock().await;
                            let response = structs::Packet { 
                                t: "init".to_string(),
                                d: structs::Packets::InitResponse(
                                    structs::BoardInitResponse {
                                        board: board.get_board(),
                                    }
                                )
                            };
                            let response = serde_json::ser::to_string(&response).unwrap();
                            websocket(&stream)
                                .write_message(tungstenite::Message::Text(response))
                                .unwrap();
                        }
                        structs::Packets::Paint(p) => {
                            let mut board = BOARD.lock().await;
                            board.set(p.x, p.y, p.colour);
                            let response = structs::Packet {
                                t: "update".to_string(),
                                d: structs::Packets::Update(structs::BoardUpdate {
                                    x: p.x,
                                    y: p.y,
                                    colour: p.colour,
                                })
                            };
                            let response = serde_json::ser::to_string(&response).unwrap();
                            let mut clients = CLIENTS.lock().await;
                            for client in clients.iter_mut() {
                                websocket(&client.socket)
                                    .write_message(tungstenite::Message::Text(response.clone()))
                                    .unwrap();
                            }
                            drop(clients);
                        }
                        _ => {
                            println!("[WebSocket] Unknown packet type");
                        }
                    }
                }
                _ => {
                    println!(
                        "[WebSocket] Error on connection from {}",
                        socket.peer_addr().unwrap()
                    );
                    websocket(&stream)
                        .write_message(tungstenite::Message::Close(None))
                        .unwrap();
                }
            },
            Err(e) => {
                println!("[WebSocket] Error reading from socket: {}", e);
                websocket(&stream).close(None).unwrap();
                break;
            }
        }
    }
}

fn websocket(raw: &TcpStream) -> tungstenite::WebSocket<TcpStream> {
    tungstenite::WebSocket::from_raw_socket(
        raw.try_clone().unwrap(),
        tungstenite::protocol::Role::Server,
        None,
    )
}
