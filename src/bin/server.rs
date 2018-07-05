extern crate bincode;
extern crate rand;
extern crate rusty_sword_arena;
extern crate zmq;

use rand::prelude::{Rng, thread_rng, ThreadRng};
use rusty_sword_arena::{
    net,
    timer,
};
use rusty_sword_arena::game::{
    Color,
    Floatable,
    GameControlMsg,
    GameSetting,
    GameState,
    PlayerEvent,
    PlayerInput,
    PlayerState,
    Vector2,
};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::thread::{self};
use zmq::Socket;

use bincode::{serialize, deserialize};

struct ColorPicker {
    // Names of the colors
    color_map : HashMap<Color, String>,
    // Colors to take
    available_colors : Vec<Color>,
}

impl ColorPicker {
    fn new() -> Self {
        let mut color_map = HashMap::<Color, String>::with_capacity(32);
        // 32 of the colors from http://eastfarthing.com/blog/2016-09-19-palette/ on 2018-06-26
        color_map.insert(Color {r : 0.021, g : 0.992, b : 0.757}, "bright teal".to_string());
        color_map.insert(Color {r : 0.630, g : 0.370, b : 0.189}, "earth".to_string());
        color_map.insert(Color {r : 0.177, g : 0.519, b : 0.189}, "tree green".to_string());
        color_map.insert(Color {r : 0.004, g : 0.718, b : 0.086}, "green".to_string());
        color_map.insert(Color {r : 0.314, g : 0.992, b : 0.204}, "poison green".to_string());
        color_map.insert(Color {r : 0.021, g : 0.861, b : 0.865}, "aqua blue".to_string());
        color_map.insert(Color {r : 0.287, g : 0.623, b : 0.666}, "turquoise blue".to_string());
        color_map.insert(Color {r : 0.187, g : 0.429, b : 0.510}, "sea blue".to_string());
        color_map.insert(Color {r : 0.223, g : 0.580, b : 0.842}, "azure".to_string());
        color_map.insert(Color {r : 0.471, g : 0.805, b : 0.971}, "lightblue".to_string());
        color_map.insert(Color {r : 0.734, g : 0.774, b : 0.924}, "light periwinkle".to_string());
        color_map.insert(Color {r : 0.555, g : 0.551, b : 0.989}, "lavender blue".to_string());
        color_map.insert(Color {r : 0.121, g : 0.393, b : 0.955}, "bright blue".to_string());
        color_map.insert(Color {r : 0.641, g : 0.554, b : 0.708}, "heather".to_string());
        color_map.insert(Color {r : 0.961, g : 0.719, b : 0.953}, "light lavendar".to_string());
        color_map.insert(Color {r : 0.874, g : 0.435, b : 0.945}, "light magenta".to_string());
        color_map.insert(Color {r : 0.657, g : 0.194, b : 0.933}, "electric purple".to_string());
        color_map.insert(Color {r : 0.212, g : 0.066, b : 0.887}, "strong blue".to_string());
        color_map.insert(Color {r : 0.575, g : 0.153, b : 0.305}, "berry".to_string());
        color_map.insert(Color {r : 0.897, g : 0.494, b : 0.640}, "dull pink".to_string());
        color_map.insert(Color {r : 0.931, g : 0.164, b : 0.069}, "tomato red".to_string());
        color_map.insert(Color {r : 0.617, g : 0.158, b : 0.123}, "brownish red".to_string());
        color_map.insert(Color {r : 0.494, g : 0.458, b : 0.104}, "ugly brown".to_string());
        color_map.insert(Color {r : 0.636, g : 0.684, b : 0.135}, "puke green".to_string());
        color_map.insert(Color {r : 0.878, g : 0.960, b : 0.247}, "sickly yellow".to_string());
        color_map.insert(Color {r : 0.872, g : 0.724, b : 0.728}, "pinkish grey".to_string());
        color_map.insert(Color {r : 0.931, g : 0.754, b : 0.569}, "light peach".to_string());
        color_map.insert(Color {r : 0.757, g : 0.566, b : 0.162}, "ochre".to_string());
        color_map.insert(Color {r : 0.972, g : 0.794, b : 0.102}, "golden yellow".to_string());
        color_map.insert(Color {r : 0.917, g : 0.475, b : 0.143}, "orange".to_string());
        color_map.insert(Color {r : 0.804, g : 100.0, b : 0.942}, "eggshell blue".to_string());
        color_map.insert(Color {r : 1.000, g : 0.982, b : 0.776}, "egg shell".to_string());

        /* Too dark, too close to another color, too gray, or I just didn't like it.
        color_map.insert(Color {r : 0.000, g : 0.000, b : 0.000}, "black".to_string());
        color_map.insert(Color {r : 0.198, g : 0.684, b : 0.531}, "green blue".to_string());
        color_map.insert(Color {r : 0.110, g : 0.203, b : 0.167}, "charcoal".to_string());
        color_map.insert(Color {r : 0.167, g : 0.323, b : 0.100}, "navy green".to_string());
        color_map.insert(Color {r : 0.147, g : 0.279, b : 0.494}, "cobalt".to_string());
        color_map.insert(Color {r : 0.449, g : 0.385, b : 0.623}, "dark lavender".to_string());
        color_map.insert(Color {r : 0.142, g : 0.072, b : 0.404}, "dark indigo".to_string());
        color_map.insert(Color {r : 0.498, g : 0.139, b : 0.528}, "darkish purple".to_string());
        color_map.insert(Color {r : 0.278, g : 0.105, b : 0.228}, "aubergine".to_string());
        color_map.insert(Color {r : 0.306, g : 0.130, b : 0.102}, "chocolate brown".to_string());
        color_map.insert(Color {r : 0.356, g : 0.316, b : 0.348}, "purplish brown".to_string());
        color_map.insert(Color {r : 0.371, g : 0.303, b : 0.159}, "mud brown".to_string());
        color_map.insert(Color {r : 0.671, g : 0.548, b : 0.462}, "pale brown".to_string());
        color_map.insert(Color {r : 0.668, g : 0.730, b : 0.702}, "silver".to_string());
        color_map.insert(Color {r : 0.519, g : 0.574, b : 0.425}, "green grey".to_string());
        color_map.insert(Color {r : 0.219, g : 0.447, b : 0.382}, "blue green".to_string());
        color_map.insert(Color {r : 0.593, g : 0.407, b : 0.466}, "mauve".to_string());
        color_map.insert(Color {r : 0.866, g : 0.219, b : 0.355}, "pink red".to_string());
        color_map.insert(Color {r : 0.945, g : 0.503, b : 0.443}, "salmon".to_string());
        color_map.insert(Color {r : 0.835, g : 0.188, b : 0.615}, "purpley pink".to_string());
        color_map.insert(Color {r : 0.634, g : 0.819, b : 0.558}, "light grey green".to_string());
        color_map.insert(Color {r : 1.000, g : 1.000, b : 1.000}, "white".to_string());
        */

        // Who knows what order we'll get stuff!  How exciting!
        let available_colors : Vec<Color> = color_map.keys().map(|&k| k.clone()).collect();

        Self { color_map, available_colors }
    }
    fn pop_color(&mut self) -> Color {
        self.available_colors.pop().unwrap()
    }
    fn push_color(&mut self, color : Color) {
        self.available_colors.push(color)
    }
    fn name_of(&self, color : Color) -> String {
        if let Some(name) = self.color_map.get(&color) {
            return name.clone();
        }
        "Unknown color".to_string()
    }
}

// Returns whether or not the player was actually there to be removed
fn remove_player(
    id : u8,
    player_states : &mut HashMap<u8, PlayerState>,
    color_picker : &mut ColorPicker,
    forced : bool,
) -> bool {
    let mut msg = format!("Player {} {}", id, if forced {"kicked for idling"} else {"left"});
    if let Some(player_state) = player_states.remove(&id) {
        msg.push_str(&format!(", name: {}, color: {:?}", player_state.name, player_state.color));
        color_picker.push_color(player_state.color);
        println!("{}", msg);
        return true;
    }
    println!("{}", msg);
    return false;
}

fn process_game_control_requests(
    game_control_server_socket : &mut Socket,
    game_setting : &mut GameSetting,
    player_states : &mut HashMap<u8, PlayerState>,
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
                        let mut id : u8 = 0;
                        loop {
                            // Is the game full?
                            if player_states.len() >= game_setting.max_players as usize {
                                println!("Join Failed: No room for for {} - max players reached.", name);
                                break;
                            }
                            // Is the name already taken?
                            if player_states
                                .values()
                                .map(|player_state | {&player_state.name})
                                .any(|x| { x == &name }) {
                                println!("Join Failed: Name \"{}\" is already taken.", name);
                                break;
                            }
                            // Find an unused, non-zero id
                            loop {
                                id = rng.gen::<u8>();
                                if (id != 0) && !player_states.contains_key(&id) { break }
                            }
                            // Assign player a color
                            let color = color_picker.pop_color();
                            // Create the new player state
                            let mut player_state = PlayerState::new(
                                &game_setting,
                                id,
                                name.clone(),
                                color,
                                Vector2::new_in_square(0.6, rng));
                            player_states.insert(id, player_state);
                            println!("Joined: {} (id {}, {})", name, id, color_picker.name_of(color));
                            break;
                        }
                        game_control_server_socket.send(&serialize(&id).unwrap(), 0).unwrap();
                    },
                    GameControlMsg::Leave {id} => {
                        // your_player_id has no meaning in this response, so we set it to the
                        // invalid id.
                        let succeeded = remove_player(id, player_states, color_picker, false);
                        // Per ZMQ REQ/REP protocol we must respond no matter what, so even invalid
                        // requests get the game settings back.
                        game_control_server_socket.send(&serialize(&succeeded).unwrap(), 0).unwrap();
                    },
                    GameControlMsg::Fetch => {
                        game_control_server_socket.send(&serialize(&game_setting).unwrap(), 0).unwrap();
                        println!("A player fetches new settings.");
                    },
                }
            },
        }
    }
}

fn coalesce_player_input(
    player_input_server_socket : &mut Socket,
    player_states : &mut HashMap<u8, PlayerState>,
    player_inputs : &mut HashMap<u8, PlayerInput>,
) {
    while let Ok(bytes) = player_input_server_socket.recv_bytes(0) {
        let player_input : PlayerInput = deserialize(&bytes[..]).unwrap();
        if let Some(player_state) = player_states.get_mut(&player_input.id) {
            player_state.drop_timer.reset();
        }
        if player_inputs.contains_key(&player_input.id) {
            player_inputs.get_mut(&player_input.id).unwrap().coalesce(player_input);
        } else {
            player_inputs.insert(player_input.id, player_input);
        }
    }
}

fn update_state(
    player_states : &mut HashMap<u8, PlayerState>,
    player_inputs : &mut HashMap<u8, PlayerInput>,
    game_setting : &mut GameSetting,
    color_picker : &mut ColorPicker,
    delta : Duration,
    rng : &mut ThreadRng,
) {
    let delta_f32 = delta.f32();

    // Update player timers, spawn anyone who is ready
    // See if any players disconnect, die, or spawn
    for (id, player_state) in player_states.iter_mut() {
        // First update any delta-dependent state
        player_state.update(delta);
        // Anyone ready to spawn?
        if player_state.dead && player_state.respawn_timer.ready {
            player_state.respawn(
                Vector2::new_in_square(0.9, rng),
                &format!("Player {} spawns", id));
        }
    }

    // Process input to affect velocities
    for (id, player_input) in &mut player_inputs.iter() {
        if let Some(player_state) = player_states.get_mut(id) {
            // Ignore input from dead players
            if player_state.dead {
                continue;
            }
            // Instantaneously face a direction
            player_state.direction = player_input.direction;
            // Update current velocity
            let clamped_move_amount = player_input.move_amount.clamped_to_normal();
            if clamped_move_amount.magnitude() > game_setting.move_threshold {
                // Player is moving -- add input to current velocity
                player_state.velocity = player_state.velocity + (clamped_move_amount * game_setting.acceleration * delta_f32);
            } else {
                // Player is holding still, apply drag to current velocity
                player_state.velocity = player_state.velocity * (1.0 - (game_setting.drag * delta_f32));
            }
            player_state.velocity = player_state.velocity.clamped_to(game_setting.max_velocity);
        }
    }
    // Process all the velocities to affect position (not just the players who had input this loop)
    for (id, player_state) in player_states.iter_mut() {
        // Dead players don't move
        if player_state.dead { continue; }
        // Apply velocity to position
        player_state.pos = player_state.pos + (player_state.velocity * delta_f32);
        // Don't go into the dark!
        if player_state.pos.x < -1.0
            || player_state.pos.x > 1.0
            || player_state.pos.y < -1.0
            || player_state.pos.y > 1.0 {
            player_state.die(&format!("Player {} was eaten by a grue.  Should have stayed in the light.", id));
        }
    }

    // Get everyone who wants to attack
    let mut attacking_ids : Vec<u8> = vec![];
    for (id, player_input) in player_inputs.iter_mut() {
        // first we need to figure out who is trying to attack, and turn off their sticky attack bool
        if player_input.attack {
            attacking_ids.push(*id);
            player_input.attack = false;
        }
    }
    // Try to attack
    for id in attacking_ids {
        let mut attacker : PlayerState;
        if let Some(maybe_attacker) = player_states.remove(&id) {
            attacker = maybe_attacker;
        } else {
            // ZeroMQ lets clients that were connected to a previous server keep sending input. /facepalm
            continue;
        }
        // Dead players don't attack
        if attacker.dead {
            player_states.insert(id, attacker);
            continue
        }
        // You can only attack so often
        if !attacker.weapon.attack_timer.ready {
            player_states.insert(id, attacker);
            continue;
        }
        // Actually attack defenders
        attacker.weapon.attack_timer.reset();
        let mut missed = true;
        for (&defender_id, defender) in player_states.iter_mut() {
            // Dead players don't defend
            if defender.dead { continue }
            if attacker.pos.distance_between(defender.pos) <= attacker.weapon.radius + game_setting.player_radius {
                missed = false;
                defender.health -= attacker.weapon.damage;
                attacker.player_events.push(PlayerEvent::AttackHit { id: defender_id });
                defender.player_events.push(PlayerEvent::TookDamage);
                println!("Player {} swings and hits ({}) for {:2.1} damage bringing him to {} health.", id, defender_id, attacker.weapon.damage, defender.health);
            }
        }
        if missed {
            attacker.player_events.push(PlayerEvent::AttackMiss);
            println!("Player {} swings...and MISSES!", id);
        }
        player_states.insert(id, attacker);
    }

    // See if any players disconnect or die
    let to_process = player_states.drain().collect::<Vec<(u8, PlayerState)>>();
    for (id, mut player_state) in to_process {
        // Mark any player for disconnection who stopped sending us input for too long
        if player_state.drop_timer.ready {
            remove_player(id, player_states, color_picker, true);
            continue;
        }
        // Anyone alive whose health went negative dies
        if !player_state.dead && player_state.health < 0.0 {
            player_state.die(&format!("Player {} dies from his wounds.", id));
        }
        player_states.insert(id, player_state);
    }
}

fn main() {
    let ctx = zmq::Context::new();

    let mut game_control_server_socket = ctx.socket(zmq::REP).unwrap();
    game_control_server_socket.set_rcvtimeo(0).unwrap();
    game_control_server_socket.bind(&format!("tcp://*:{}", net::GAME_CONTROL_PORT)).unwrap();

    let game_state_server_socket = ctx.socket(zmq::PUB).unwrap();
    game_state_server_socket.bind(&format!("tcp://*:{}", net::GAME_STATE_PORT)).unwrap();

    let mut player_input_server_socket = ctx.socket(zmq::PULL).unwrap();
    player_input_server_socket.set_rcvtimeo(0).unwrap();
    player_input_server_socket.bind(&format!("tcp://*:{}", net::PLAYER_INPUT_PORT)).unwrap();

    let mut loop_iterations : i64 = 0;
    let mut loop_start = Instant::now();
    let mut frame_timer = timer::Timer::from_nanos(16666666); // 60 FPS
    let mut color_picker = ColorPicker::new();
    let mut game_setting = GameSetting::new();
    let mut rng = thread_rng();
    let mut frame_number : u64 = 0;
    let mut player_states = HashMap::<u8, PlayerState>::new();
    let mut player_inputs = HashMap::<u8, PlayerInput>::new();

    println!("--------------------------------------------------------------");
    println!("Server started (Ctrl-C to stop)\n{:#?}", game_setting);
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

        // Handle and coalesce all the player input we've received so far into player_inputs
        coalesce_player_input(&mut player_input_server_socket, &mut player_states, &mut player_inputs);

        // Move, attack, etc.
        update_state(&mut player_states, &mut player_inputs, &mut game_setting, &mut color_picker, delta, &mut rng);

        // Process a frame (if it's time)

        if frame_timer.ready {
            frame_timer.reset();
            // Broadcast new game state computed this frame
            if frame_number % 1800 == 0 {
                let status = format!(
                    "STATUS: Frame: {}, Loops this frame: {}", frame_number, loop_iterations);
                println!("{}", status);
            }
            let game_state = GameState {
                frame_number,
                delta,
                game_setting_hash : game_setting.get_hash(),
                player_states : player_states.clone(),
            };
            game_state_server_socket.send(&serialize(&game_state).unwrap(), 0).unwrap();
            for player_state in player_states.values_mut() {
                player_state.new_frame();
            }
            loop_iterations = 0;
            frame_number += 1;
        }
    }
}
