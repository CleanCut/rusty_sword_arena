extern crate bincode;
extern crate rand;
extern crate rusty_sword_arena;
#[macro_use]
extern crate serde_derive;
extern crate zmq;

use rand::prelude::{Rng, thread_rng, ThreadRng};
use rusty_sword_arena::{
    Color,
    Floatable,
    GameControlMsg,
    GameSetting,
    GameState,
    net,
    PlayerEvent,
    PlayerInput,
    PlayerSetting,
    PlayerState,
    timer,
};
use std::cell::RefCell;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::thread::{self};
use zmq::Socket;


use bincode::{serialize, deserialize};

struct ColorPicker {
    colors : HashMap<String, Color>,
}

impl ColorPicker {
    fn new() -> Self {
        let mut colors = HashMap::<String, Color>::with_capacity(32);
        // 32 of the colors from http://eastfarthing.com/blog/2016-09-19-palette/ on 2018-06-26
        colors.insert("bright teal".to_string(), Color {r : 0.021, g : 0.992, b : 0.757});
        colors.insert("earth".to_string(), Color {r : 0.630, g : 0.370, b : 0.189});
        colors.insert("tree green".to_string(), Color {r : 0.177, g : 0.519, b : 0.189});
        colors.insert("green".to_string(), Color {r : 0.004, g : 0.718, b : 0.086});
        colors.insert("poison green".to_string(), Color {r : 0.314, g : 0.992, b : 0.204});
        colors.insert("aqua blue".to_string(), Color {r : 0.021, g : 0.861, b : 0.865});
        colors.insert("turquoise blue".to_string(), Color {r : 0.287, g : 0.623, b : 0.666});
        colors.insert("sea blue".to_string(), Color {r : 0.187, g : 0.429, b : 0.510});
        colors.insert("azure".to_string(), Color {r : 0.223, g : 0.580, b : 0.842});
        colors.insert("lightblue".to_string(), Color {r : 0.471, g : 0.805, b : 0.971});
        colors.insert("light periwinkle".to_string(), Color {r : 0.734, g : 0.774, b : 0.924});
        colors.insert("lavender blue".to_string(), Color {r : 0.555, g : 0.551, b : 0.989});
        colors.insert("bright blue".to_string(), Color {r : 0.121, g : 0.393, b : 0.955});
        colors.insert("heather".to_string(), Color {r : 0.641, g : 0.554, b : 0.708});
        colors.insert("light lavendar".to_string(), Color {r : 0.961, g : 0.719, b : 0.953});
        colors.insert("light magenta".to_string(), Color {r : 0.874, g : 0.435, b : 0.945});
        colors.insert("electric purple".to_string(), Color {r : 0.657, g : 0.194, b : 0.933});
        colors.insert("strong blue".to_string(), Color {r : 0.212, g : 0.066, b : 0.887});
        colors.insert("berry".to_string(), Color {r : 0.575, g : 0.153, b : 0.305});
        colors.insert("dull pink".to_string(), Color {r : 0.897, g : 0.494, b : 0.640});
        colors.insert("tomato red".to_string(), Color {r : 0.931, g : 0.164, b : 0.069});
        colors.insert("brownish red".to_string(), Color {r : 0.617, g : 0.158, b : 0.123});
        colors.insert("ugly brown".to_string(), Color {r : 0.494, g : 0.458, b : 0.104});
        colors.insert("puke green".to_string(), Color {r : 0.636, g : 0.684, b : 0.135});
        colors.insert("sickly yellow".to_string(), Color {r : 0.878, g : 0.960, b : 0.247});
        colors.insert("pinkish grey".to_string(), Color {r : 0.872, g : 0.724, b : 0.728});
        colors.insert("light peach".to_string(), Color {r : 0.931, g : 0.754, b : 0.569});
        colors.insert("ochre".to_string(), Color {r : 0.757, g : 0.566, b : 0.162});
        colors.insert("golden yellow".to_string(), Color {r : 0.972, g : 0.794, b : 0.102});
        colors.insert("orange".to_string(), Color {r : 0.917, g : 0.475, b : 0.143});
        colors.insert("eggshell blue".to_string(), Color {r : 0.804, g : 100.0, b : 0.942});
        colors.insert("egg shell".to_string(), Color {r : 1.000, g : 0.982, b : 0.776});

        /* Too dark, too close to another color, too gray, or I just didn't like it.
        colors.insert("black".to_string(), Color {r : 0.000, g : 0.000, b : 0.000});
        colors.insert("green blue".to_string(), Color {r : 0.198, g : 0.684, b : 0.531});
        colors.insert("charcoal".to_string(), Color {r : 0.110, g : 0.203, b : 0.167});
        colors.insert("navy green".to_string(), Color {r : 0.167, g : 0.323, b : 0.100});
        colors.insert("cobalt".to_string(), Color {r : 0.147, g : 0.279, b : 0.494});
        colors.insert("dark lavender".to_string(), Color {r : 0.449, g : 0.385, b : 0.623});
        colors.insert("dark indigo".to_string(), Color {r : 0.142, g : 0.072, b : 0.404});
        colors.insert("darkish purple".to_string(), Color {r : 0.498, g : 0.139, b : 0.528});
        colors.insert("aubergine".to_string(), Color {r : 0.278, g : 0.105, b : 0.228});
        colors.insert("chocolate brown".to_string(), Color {r : 0.306, g : 0.130, b : 0.102});
        colors.insert("purplish brown".to_string(), Color {r : 0.356, g : 0.316, b : 0.348});
        colors.insert("mud brown".to_string(), Color {r : 0.371, g : 0.303, b : 0.159});
        colors.insert("pale brown".to_string(), Color {r : 0.671, g : 0.548, b : 0.462});
        colors.insert("silver".to_string(), Color {r : 0.668, g : 0.730, b : 0.702});
        colors.insert("green grey".to_string(), Color {r : 0.519, g : 0.574, b : 0.425});
        colors.insert("blue green".to_string(), Color {r : 0.219, g : 0.447, b : 0.382});
        colors.insert("mauve".to_string(), Color {r : 0.593, g : 0.407, b : 0.466});
        colors.insert("pink red".to_string(), Color {r : 0.866, g : 0.219, b : 0.355});
        colors.insert("salmon".to_string(), Color {r : 0.945, g : 0.503, b : 0.443});
        colors.insert("purpley pink".to_string(), Color {r : 0.835, g : 0.188, b : 0.615});
        colors.insert("light grey green".to_string(), Color {r : 0.634, g : 0.819, b : 0.558});
        colors.insert("white".to_string(), Color {r : 1.000, g : 1.000, b : 1.000});
        */
        Self { colors }
    }
    fn pop_color(&mut self) -> (String, Color) {
        // Lets not crash if we get emptied
        if self.colors.len() == 0 {
            return ("overflow white".to_string(), Color {r : 1.0, g : 1.0, b : 1.0});
        }
        // Who knows what order we'll get stuff in.  How exciting!
        let key = self.colors.keys().nth(0).unwrap().clone();
        self.colors.remove_entry(&key).unwrap()
    }
    fn push_color(&mut self, name : String, color : Color) {
        self.colors.insert(name, color);
    }
}

fn remove_player(
    id : u8,
    game_setting : &mut GameSetting,
    player_states : &mut HashMap<u8, RefCell<PlayerState>>,
    color_picker : &mut ColorPicker,
    forced : bool,
) {
    game_setting.your_player_id = 0;
    let mut msg = format!("Player {} {}", id, if forced {"kicked for idling"} else {"left"});
    if let Some(player_setting) = game_setting.player_settings.remove(&id) {
        msg.push_str(&format!(", name: {}", player_setting.name));
        color_picker.push_color(player_setting.name.clone(), player_setting.color);
    }
    player_states.remove(&id);
    println!("{}", msg);
}

fn process_game_control_requests(
    game_control_server_socket : &mut Socket,
    game_setting : &mut GameSetting,
    player_states : &mut HashMap<u8, RefCell<PlayerState>>,
    rng : &mut ThreadRng,
    color_picker : &mut ColorPicker,
) {
    'gamecontrol:
    loop {
        match game_control_server_socket.recv_bytes(0) {
            Err(_e) => break 'gamecontrol,
            Ok(bytes) => {
                let msg: GameControlMsg = deserialize(&bytes[..]).unwrap();
                match msg {
                    GameControlMsg::Join {name} => {
                        if game_setting.player_settings.len() < game_setting.max_players as usize {
                            // Find an unused, non-zero id
                            let mut new_id : u8;
                            loop {
                                new_id = rng.gen::<u8>();
                                if (new_id != 0) && !player_states.contains_key(&new_id) { break }
                            }
                            game_setting.your_player_id = new_id;
                            // Make sure player name is unique, and then store it.
                            let mut new_name = name.clone();
                            while game_setting.player_settings
                                .values()
                                .map(|player_setting | {&player_setting.name})
                                .any(|x| { x == &new_name }) {
                                new_name.push_str("_");
                            }
                            // Assign player a color
                            let (color_name, color) = color_picker.pop_color();
                            // Create the PlayerSetting and add it to the GameSetting
                            game_setting.player_settings.insert(
                                new_id,
                                PlayerSetting {
                                    name : new_name.clone(),
                                    color_name : color_name.clone(),
                                    color});
                            // Create the new player state
                            let mut player_state = PlayerState::new();
                            player_state.id = new_id;
                            player_states.insert(new_id, RefCell::new(player_state));
                            println!("Joined: {} (id {}, {})", new_name, new_id, color_name);
                        } else {
                            // Use the invalid player ID to let the client know they didn't get
                            // to join. Lame.
                            game_setting.your_player_id = 0;
                            println!("Denied entrance for {}", name)
                        }
                        game_control_server_socket.send(&serialize(&game_setting).unwrap(), 0).unwrap();
                    },
                    GameControlMsg::Leave {id} => {
                        // your_player_id has no meaning in this response, so we set it to the
                        // invalid id.
                        remove_player(id, game_setting, player_states, color_picker, false);
                        // Per ZMQ REQ/REP protocol we must respond no matter what, so even invalid
                        // requests get the game settings back.
                        game_control_server_socket.send(&serialize(&game_setting).unwrap(), 0).unwrap();
                    },
                    GameControlMsg::Fetch {id} => {
                        // your_player_id has no meaning in this response, so we make sure it
                        // is the invalid id.
                        if { game_setting.your_player_id != 0 } {
                            game_setting.your_player_id = 0;
                        }
                        game_control_server_socket.send(&serialize(&game_setting).unwrap(), 0).unwrap();
                        println!("Player {} fetches new settings.", id);
                    },
                }
            },
        }
    }
}

fn process_player_input(
    player_input_server_socket : &mut Socket,
    player_states : &mut HashMap<u8, RefCell<PlayerState>>,
    player_inputs : &mut HashMap<u8, PlayerInput>,
) {
    while let Ok(bytes) = player_input_server_socket.recv_bytes(0) {
        let player_input : PlayerInput = deserialize(&bytes[..]).unwrap();
        if let Some(player_state) = player_states.get(&player_input.id) {
            player_state.borrow_mut().disconnect_timer.reset();
        }
        if player_inputs.contains_key(&player_input.id) {
            player_inputs.get_mut(&player_input.id).unwrap().coalesce(player_input);
        } else {
            player_inputs.insert(player_input.id, player_input);
        }
    }
}

fn update_state(
    player_states : &mut HashMap<u8, RefCell<PlayerState>>,
    player_inputs : &mut HashMap<u8, PlayerInput>,
    game_setting : &mut GameSetting,
    color_picker : &mut ColorPicker,
    delta : Duration,
) {
    let delta_f32 = delta.f32();
    // Everyone Turns & Moves
    for (id, player_input) in &mut player_inputs.iter() {
        if let Some(player_state) = player_states.get(&player_input.id) {
            let mut player_state = player_state.borrow_mut();
            player_state.angle = player_input.turn_angle;
            player_state.pos.x += game_setting.move_speed * player_input.horiz_axis * delta_f32;
            player_state.pos.y += game_setting.move_speed * player_input.vert_axis * delta_f32;
        }
    }
    // Everyone attacks
    let mut attacking_ids : Vec<u8> = vec![];
    for (id, player_input) in player_inputs.iter_mut() {
        // first we need to figure out who is trying to attack, and turn off their sticky attack bool
        if player_input.attack {
            attacking_ids.push(*id);
            player_input.attack = false;
        }
    }
    for id in attacking_ids {
        let other_ids : Vec<u8> = player_states.keys().filter_map(|&x| if x == id {None} else {Some(x)}).collect();
        let mut attacker = player_states.get(&id).unwrap().borrow_mut();
        // TODO: Move attack_timer from player to weapon
        if !attacker.attack_timer.ready {
            continue;
        }
        let mut missed = true;
        for other_id in other_ids {
            let mut defender = player_states.get(&other_id).unwrap().borrow_mut();
            println!("Distance: {:2.2}, Weapon Radius: {:2.2}", attacker.pos.distance_between(defender.pos), attacker.weapon.radius + game_setting.player_radius);
            if attacker.pos.distance_between(defender.pos) <= attacker.weapon.radius + game_setting.player_radius {
                missed = false;
                defender.health -= attacker.weapon.damage;
                attacker.player_events.push(PlayerEvent::AttackHit { id: other_id });
                defender.player_events.push(PlayerEvent::TookDamage);
                println!("({}) hit ({}) for {:2.1} damage bringing him to {} health.", id, other_id, attacker.weapon.damage, defender.health);
            }
            if missed {
                attacker.player_events.push(PlayerEvent::AttackMiss);
            }
        }
    }

    // See if we need to disconnect, kill, or spawn anyone
    let mut players_to_disconnect : Vec<u8> = vec![];
    for (id, player_state) in player_states.iter_mut() {
        let mut player_state = player_state.borrow_mut();
        // First update any delta-dependent state
        player_state.update(delta);
        // Mark any player for disconnection who stopped sending us input for too long
        if player_state.disconnect_timer.ready {
            players_to_disconnect.push(*id);
        }
        //
    }
    // Actually disconnect
    for id in players_to_disconnect {
        remove_player(id, game_setting, player_states, color_picker, true)
    }
}

fn main() {
    let ctx = zmq::Context::new();

    let mut game_control_server_socket = ctx.socket(zmq::REP).unwrap();
    game_control_server_socket.set_rcvtimeo(0).unwrap();
    game_control_server_socket.bind(&format!("tcp://*:{}", net::GAME_CONTROL_PORT)).unwrap();

    let mut game_state_server_socket = ctx.socket(zmq::PUB).unwrap();
    game_state_server_socket.bind(&format!("tcp://*:{}", net::GAME_STATE_PORT)).unwrap();

    let mut player_input_server_socket = ctx.socket(zmq::PULL).unwrap();
    player_input_server_socket.set_rcvtimeo(0).unwrap();
    player_input_server_socket.bind(&format!("tcp://*:{}", net::PLAYER_INPUT_PORT)).unwrap();

    let mut loop_iterations : i64 = 0;
    let mut processed = 0;
    let mut loop_start = Instant::now();
    let mut frame_timer = timer::Timer::from_nanos(16666666); // 60 FPS
    let mut color_picker = ColorPicker::new();

    let mut game_setting = GameSetting {
        your_player_id : 0,
        max_players : 64,
        player_radius : 0.05,
        move_speed : 0.4,
        move_dampening : 0.5,
        frame_delay : 0.5,
        respawn_delay : 5.0,
        drop_timeout : 10.0,
        player_settings : HashMap::<u8, PlayerSetting>::new(),
    };

    let mut rng = thread_rng();
    let mut frame_number : u64 = 0;
    let mut player_states = HashMap::<u8, RefCell<PlayerState>>::new();
    let mut player_inputs = HashMap::<u8, PlayerInput>::new();

    'gameloop:
    loop {
        let delta = loop_start.elapsed();
        loop_start = Instant::now();
        frame_timer.update(delta);
        // TODO: Refactor the server to be interrupt-driven, so we don't have to sleep to keep a
        //       busy-loop from sucking up 100% of a CPU
        thread::sleep(Duration::from_micros(100));
        loop_iterations += 1;

        // Handle and reply to all Game Control requests. The game settings might get changed.
        process_game_control_requests(
            &mut game_control_server_socket,
            &mut game_setting,
            &mut player_states,
            &mut rng,
            &mut color_picker,
        );

        // Handle and process all the player input we've received so far
        process_player_input(&mut player_input_server_socket, &mut player_states,&mut player_inputs);

        update_state(&mut player_states, &mut player_inputs, &mut game_setting, &mut color_picker, delta);

        // Process a frame (if it's time)

        if frame_timer.ready {
            frame_timer.reset();
            // Broadcast new game state computed this frame
            let status = format!("STATUS | Frame: {}, Delta: {:2.3}, Messages Processed: {}, Loops: {}",
                                 frame_number, delta.f32(), processed, loop_iterations);
            let game_state = GameState {
                frame_number,
                delta,
                game_setting_hash : game_setting.get_hash(),
                player_states : player_states.iter().map(|(&k, ref v)| (k, v.borrow().clone())).collect::<HashMap<u8, PlayerState>>(),
            };
            game_state_server_socket.send(&serialize(&game_state).unwrap(), 0).unwrap();
            processed = 0;
            loop_iterations = 0;
            frame_number += 1;
        }
    }

    // Time to shut down
    thread::sleep(Duration::from_secs(1));
}
