extern crate rusty_sword_arena;
extern crate zmq;

use std::time::{Duration, Instant};
use std::thread::{self};

use rusty_sword_arena as rsa;

fn main() {
    let ctx = zmq::Context::new();

    let mut req_socket = ctx.socket(zmq::REQ).unwrap();
    let mut sub_socket = ctx.socket(zmq::SUB).unwrap();
    req_socket.connect(&format!("tcp://localhost:{}", rsa::client_port)).unwrap();
    sub_socket.connect(&format!("tcp://localhost:{}", rsa::game_port)).unwrap();
    sub_socket.set_subscribe(&[]);

    loop {
        req_socket.send_str("hello from client", 0).unwrap();
        println!("Sent hello, received: {}", req_socket.recv_string(0).unwrap().unwrap());

        println!("Game info from server: {}", sub_socket.recv_string(0).unwrap().unwrap());
        thread::sleep(Duration::from_secs(1));
    }
}
