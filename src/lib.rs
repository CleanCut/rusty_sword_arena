/// The endpoint for clients to request (zmq::REQ) to the server's reply (zmq::REP) socket.
/// Clients will send their updates to the server this way.
pub const client_port : i32 = 8001;

/// The endpoint for clients to subscribe (zmq::SUB) to the server's publish (zmq::PUB) socket and
/// receive game state updates.
pub const game_port : i32 = 8002;


pub mod prelude {
    pub use super::{client_port, game_port};
}

pub mod network {
//    extern crate zmq;
//
//    use std::time::{Duration, Instant};
//    use std::thread::{self};
//
//    let ctx = zmq::Context::new();
//
//    let mut req_socket = ctx.socket(zmq::REQ).unwrap();
//    let mut sub_socket = ctx.socket(zmq::SUB).unwrap();
//    req_socket.connect(&format!("tcp://localhost:{}", rsa::client_port)).unwrap();
//    sub_socket.connect(&format!("tcp://localhost:{}", rsa::game_port)).unwrap();
//    sub_socket.set_subscribe(&[]);
//
//    loop {
//    req_socket.send_str("hello from client", 0).unwrap();
//    println!("Sent hello, received: {}", req_socket.recv_string(0).unwrap().unwrap());
//
//    println!("Game info from server: {}", sub_socket.recv_string(0).unwrap().unwrap());
//    thread::sleep(Duration::from_secs(1));
//    }
}