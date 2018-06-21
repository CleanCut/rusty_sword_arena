extern crate bincode;
extern crate rand;
extern crate rusty_sword_arena;
#[macro_use]
extern crate serde_derive;
extern crate zmq;

use rand::prelude::{Rng, thread_rng};
use rsa::{Color, GameControlMsg, GameSettings};
use rusty_sword_arena as rsa;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::thread::{self};


use bincode::{serialize, deserialize};

struct ExtraPlayerState {
    horiz_axis : f32,
    vert_axis : f32,
    attack_timer : f32,
}

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

    let mut gs = GameSettings {
        your_player_id : 0,
        max_players : 64,
        player_radius : 0.05,
        move_speed : 0.001,
        move_dampening : 0.5,
        frame_delay : 0.5,
        respawn_delay : 5.0,
        drop_timeout : 10.0,
        player_names : HashMap::<u8, String>::new(),
        player_colors : HashMap::<u8, Color>::new(),
    };
    println!("{:?}", gs);

    let mut rng = thread_rng();

    'gameloop:
    loop {
        thread::sleep(Duration::from_millis(1));
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
                        GameControlMsg::Join {name} => {
                            if gs.player_names.len() < gs.max_players as usize {
                                // Find an unused, non-zero id
                                let mut new_id : u8;
                                loop {
                                    new_id = rng.gen::<u8>();
                                    if (new_id != 0) && !gs.player_names.contains_key(&new_id) { break }
                                }
                                gs.your_player_id = new_id;
                                // Make sure player name is unique, and then store it.
                                let mut new_name = name.clone();
                                while gs.player_names.values().any(|x| { x == &new_name }) {
                                    new_name.push_str("_");
                                }
                                gs.player_names.insert(new_id, new_name.clone());
                                // Assign player a color
                                let new_color = Color { r: 1.0, g: 0.0, b: 0.0 };
                                gs.player_colors.insert(new_id, new_color.clone());
                                println!("Joined: {} (id {}, {:?})", new_name, new_id, new_color);
                            } else {
                                // Use the invalid player ID to let the client know they didn't get
                                // to join. Lame.
                                gs.your_player_id = 0;
                                println!("Denied entrance for {}", name)
                            }
                            game_control_server_socket.send(&serialize(&gs).unwrap(), 0).unwrap();
                        },
                        GameControlMsg::Leave {id} => {
                            // your_player_id has no meaning in this response, so we set it to the
                            // invalid id.
                            gs.your_player_id = 0;
                            if !gs.player_names.contains_key(&id) {
                                println!("Ignoring request for player {} to leave since that player isn't here.", id);
                            } else {
                                let name = gs.player_names.remove(&id).unwrap();
                                gs.player_colors.remove(&id);
                                println!("Player {} ({}) leaves", name, id);
                            }
                            // Per ZMQ REQ/REP protocol we must respond no matter what, so even invalid
                            // requests get the game settings back.
                            game_control_server_socket.send(&serialize(&gs).unwrap(), 0).unwrap();
                        },
                        GameControlMsg::Fetch => {
                            // your_player_id has no meaning in this response, so we set it to the
                            // invalid id.
                            gs.your_player_id = 0;
                            game_control_server_socket.send(&serialize(&gs).unwrap(), 0).unwrap();
                            println!("Someone fetches new settings.");
                        },
                    }
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
