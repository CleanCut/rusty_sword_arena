extern crate bincode;
extern crate rand;
extern crate rusty_sword_arena;
#[macro_use]
extern crate serde_derive;
extern crate zmq;

use rand::prelude::{Rng, thread_rng, ThreadRng};
use rsa::{Color, GameControlMsg, GameSettings, GameState, PlayerInput, PlayerState};
use rusty_sword_arena as rsa;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::thread::{self};
use zmq::Socket;


use bincode::{serialize, deserialize};

fn process_game_control_requests(
    game_control_server_socket : &mut Socket,
    game_settings : &mut GameSettings,
    player_states : &mut HashMap<u8, PlayerState>,
    rng : &mut ThreadRng) -> bool {
    let mut game_settings_changed = false;
    'gamecontrol:
    loop {
        match game_control_server_socket.recv_bytes(0) {
            Err(e) => break 'gamecontrol,
            Ok(bytes) => {
                let msg: GameControlMsg = deserialize(&bytes[..]).unwrap();
                match msg {
                    GameControlMsg::Join {name} => {
                        if game_settings.player_names.len() < game_settings.max_players as usize {
                            // Find an unused, non-zero id
                            let mut new_id : u8;
                            loop {
                                new_id = rng.gen::<u8>();
                                if (new_id != 0) && !game_settings.player_names.contains_key(&new_id) { break }
                            }
                            game_settings.your_player_id = new_id;
                            // Make sure player name is unique, and then store it.
                            let mut new_name = name.clone();
                            while game_settings.player_names.values().any(|x| { x == &new_name }) {
                                new_name.push_str("_");
                            }
                            game_settings.player_names.insert(new_id, new_name.clone());
                            // Assign player a color
                            let new_color = Color { r: 1.0, g: 0.0, b: 0.0 };
                            game_settings.player_colors.insert(new_id, new_color.clone());
                            // Create the new player state
                            //player_
                            println!("Joined: {} (id {}, {:?})", new_name, new_id, new_color);
                        } else {
                            // Use the invalid player ID to let the client know they didn't get
                            // to join. Lame.
                            game_settings.your_player_id = 0;
                            println!("Denied entrance for {}", name)
                        }
                        game_settings_changed = true;
                        game_control_server_socket.send(&serialize(&game_settings).unwrap(), 0).unwrap();
                    },
                    GameControlMsg::Leave {id} => {
                        // your_player_id has no meaning in this response, so we set it to the
                        // invalid id.
                        game_settings.your_player_id = 0;
                        if !game_settings.player_names.contains_key(&id) {
                            println!("Ignoring request for player {} to leave since that player isn't here.", id);
                        } else {
                            let name = game_settings.player_names.remove(&id).unwrap();
                            game_settings.player_colors.remove(&id);
                            println!("Player {} ({}) leaves", name, id);
                        }
                        // Per ZMQ REQ/REP protocol we must respond no matter what, so even invalid
                        // requests get the game settings back.
                        game_settings_changed = true;
                        game_control_server_socket.send(&serialize(&game_settings).unwrap(), 0).unwrap();
                    },
                    GameControlMsg::Fetch => {
                        // your_player_id has no meaning in this response, so we make sure it
                        // is the invalid id.
                        if { game_settings.your_player_id != 0 } {
                            game_settings.your_player_id = 0;
                            game_settings_changed = true;
                        }
                        game_control_server_socket.send(&serialize(&game_settings).unwrap(), 0).unwrap();
                        println!("Someone fetches new settings.");
                    },
                }
            },
        }
    }
    game_settings_changed
}

fn process_player_input(player_input_server_socket : &mut Socket, player_states : &mut HashMap<u8, PlayerState>) {
    while let Ok(bytes) = player_input_server_socket.recv_bytes(0) {
        let player_input : PlayerInput = deserialize(&bytes[..]).unwrap();
        println!("{:#?}", player_input);
    }
}

fn main() {
    let ctx = zmq::Context::new();

    let mut game_control_server_socket = ctx.socket(zmq::REP).unwrap();
    game_control_server_socket.set_rcvtimeo(0).unwrap();
    game_control_server_socket.bind(&format!("tcp://*:{}", rsa::net::GAME_CONTROL_PORT)).unwrap();

    let mut game_state_server_socket = ctx.socket(zmq::PUB).unwrap();
    game_state_server_socket.bind(&format!("tcp://*:{}", rsa::net::GAME_STATE_PORT)).unwrap();

    let mut player_input_server_socket = ctx.socket(zmq::PULL).unwrap();
    player_input_server_socket.set_rcvtimeo(0).unwrap();
    player_input_server_socket.bind(&format!("tcp://*:{}", rsa::net::PLAYER_INPUT_PORT)).unwrap();

    let mut loop_iterations : i64 = 0;
    let mut processed = 0;
    let mut report_starttime = Instant::now();
    let report_frequency = Duration::new(1, 0);

    let mut game_settings = GameSettings {
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

    let mut rng = thread_rng();
    let mut game_settings_changed = true;
    let mut frame_number : u64 = 0;
    let mut player_states = HashMap::<u8, PlayerState>::new();

    'gameloop:
    loop {
        thread::sleep(Duration::from_micros(100));
        loop_iterations += 1;

        // Handle and reply to all Game Control requests. The game settings might get changed.
        game_settings_changed = game_settings_changed || process_game_control_requests(
            &mut game_control_server_socket,
            &mut game_settings,
            &mut player_states,
            &mut rng);

        // Handle and process all the player input we've received so far
        process_player_input(&mut player_input_server_socket, &mut player_states);

        // Process a frame (if it's time)
        let delta = report_starttime.elapsed();
        if delta > report_frequency {
            report_starttime = Instant::now();
            // Convert delta to a float
            let delta = delta.as_secs() as f32 + delta.subsec_nanos() as f32 * 1e-9;

            // Broadcast new game state computed this frame
            let status = format!("STATUS | Frame: {}, Delta: {:2.3}, Messages Processed: {}, Loops: {}",
                                 frame_number, delta, processed, loop_iterations);
            let game_state = GameState {
                frame_number,
                delta,
                game_settings_changed,
                player_states : player_states.clone(),
            };
            game_state_server_socket.send(&serialize(&game_state).unwrap(), 0).unwrap();
            println!("{}", status);
            game_settings_changed = false;
            processed = 0;
            loop_iterations = 0;
            frame_number += 1;
        }
    }

    // Time to shut down
    thread::sleep(Duration::from_secs(1));
}
