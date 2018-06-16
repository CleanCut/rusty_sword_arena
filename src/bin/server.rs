extern crate rusty_sword_arena;
extern crate zmq;

use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::thread::{self};

use rusty_sword_arena as rsa;

use rsa::{Color, GameControlMsg, GameSettings};

#[macro_use]
extern crate serde_derive;
extern crate bincode;

use bincode::{serialize, deserialize};

fn main() {
    let ctx = zmq::Context::new();

    let mut game_control_server_socket = ctx.socket(zmq::REP).unwrap();
    game_control_server_socket.set_rcvtimeo(0);
    game_control_server_socket.bind(&format!("tcp://*:{}", rsa::net::GAME_CONTROL_PORT)).unwrap();

    let mut game_state_server_socket = ctx.socket(zmq::PUB).unwrap();
    game_state_server_socket.bind(&format!("tcp://*:{}", rsa::net::GAME_STATE_PORT)).unwrap();

    let mut player_input_server_socket = ctx.socket(zmq::PULL).unwrap();
    player_input_server_socket.bind(&format!("tcp://*:{}", rsa::net::PLAYER_INPUT_PORT)).unwrap();

    let mut loop_iterations : i64 = 0;
    let mut processed = 0;
    let mut report_starttime = Instant::now();
    let report_frequency = Duration::new(1, 0);

    let mut game_settings = GameSettings {
        your_player_id : 0,
        player_radius : 0.05,
        move_speed : 0.001,
        move_dampening : 0.5,
        frame_delay : 0.5,
        respawn_delay : 5.0,
        drop_timeout : 10.0,
        player_names : HashMap::<u8, String>::new(),
        player_colors : HashMap::<u8, Color>::new(),
    };
    println!("{:?}", game_settings);

    // Reusable zmq message container -- to avoid unnecessary extra allocations
    let mut msg = zmq::Message::new();

    'gameloop:
    loop {
        loop_iterations += 1;

        // Reply to all Game Control requests
        'gamecontrol:
        loop {
            match game_control_server_socket.recv_bytes(0) {
                Err(e) => break 'gamecontrol,
                Ok(bytes) => {
                    let msg: GameControlMsg = deserialize(&bytes[..]).unwrap();
                    println!("{:?}", msg);
                    match msg {
                        GameControlMsg::Join {name} => println!("{} joins", name),
                        GameControlMsg::Leave {name} => println!("{} leaves", name),
                        GameControlMsg::Fetch => println!("Someone fetches new settings."),
                    }
                    game_control_server_socket.send(&serialize(&game_settings).unwrap(), 0).unwrap();
                },
            }
        }

        // Pull all the Player Input pushes

        // Process a frame (if it's time)
        if report_starttime.elapsed() > report_frequency {

        // Broadcast new game state computed this frame
            let status = format!("STATUS | Time: {:?}, Messages Processed: {}, Loops: {}",
                                 report_starttime, processed, loop_iterations);
            game_state_server_socket.send_str(&status, 0);
            println!("{}", status);
            processed = 0;
            loop_iterations = 0;
            report_starttime = Instant::now();
        }
    }

    // Time to shut down
    thread::sleep(Duration::from_secs(1));
}
