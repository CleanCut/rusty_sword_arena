extern crate rusty_sword_arena;
extern crate zmq;

use std::time::{Duration, Instant};
use std::thread::{self};

use rusty_sword_arena as rsa;

fn main() {
    let ctx = zmq::Context::new();

    let mut rep_socket = ctx.socket(zmq::REP).unwrap();
    rep_socket.set_rcvtimeo(0);
    let mut pub_socket = ctx.socket(zmq::PUB).unwrap();
    rep_socket.bind(&format!("tcp://*:{}", rsa::client_port)).unwrap();
    pub_socket.bind(&format!("tcp://*:{}", rsa::game_port)).unwrap();

    let mut loop_iterations : i64 = 0;
    let mut processed = 0;
    let mut report_starttime = Instant::now();
    let report_frequency = Duration::new(1, 0);

    'gameloop:
    loop {
        loop_iterations += 1;

        // Handle all pending requests
        loop {
            if let Ok(wrapper) = rep_socket.recv_string(0) {
                // The transport worked, now let's see if it's a valid string
                if let Ok(msg) = wrapper {
                    processed += 1;
                    println!("Message received: {}", msg);
                    rep_socket.send_str("ack", 0).unwrap();
                    if msg == "shutdown" {
                        break 'gameloop;
                    }
                    continue;
                }
            }
            break;
        }

        // Broadcast the current timestamp and how many requests were processed
        if report_starttime.elapsed() > report_frequency {
            let status = format!("STATUS | Time: {:?}, Messages Processed: {}, Loops: {}",
                                 report_starttime, processed, loop_iterations);
            pub_socket.send_str(&status, 0);
            println!("{}", status);
            processed = 0;
            loop_iterations = 0;
            report_starttime = Instant::now();
        }
    }

    // Time to shut down
    pub_socket.send_str("shutdown", 0);
    thread::sleep(Duration::from_secs(1));

}