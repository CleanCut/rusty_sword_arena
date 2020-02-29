use bincode::{deserialize, serialize};
use rand::prelude::{thread_rng, Rng, ThreadRng};
use rusty_sword_arena::{
    game::{
        Floatable, GameControlMsg, GameSettings, GameState, HighScores, PlayerEvent, PlayerInput,
        PlayerState,
    },
    gfx::{clamp_vec_to_magnitude, distance, new_in_square, Color, Vec2},
    net, timer,
};
use std::collections::HashMap;
use std::thread;
use std::time::{Duration, Instant};
use zmq;

struct ColorPicker {
    index: usize,
    // Colors to take
    colors: Vec<Color>,
}

// Why not just a Vec?  Because I've reimplemented this a half-dozen times...
#[allow(clippy::excessive_precision, clippy::unreadable_literal)]
impl ColorPicker {
    fn new() -> Self {
        let colors = vec![
            // Darkest
            Color {
                r: 0.6274510,
                g: 0.5176471,
                b: 0.2666667,
            },
            Color {
                r: 0.6901961,
                g: 0.6901961,
                b: 0.6901961,
            },
            Color {
                r: 0.7215686,
                g: 0.7215686,
                b: 0.2509804,
            },
            Color {
                r: 0.7372549,
                g: 0.5490196,
                b: 0.2980392,
            },
            Color {
                r: 0.8156863,
                g: 0.5019608,
                b: 0.3607843,
            },
            Color {
                r: 0.8156863,
                g: 0.4392157,
                b: 0.4392157,
            },
            Color {
                r: 0.7529412,
                g: 0.4392157,
                b: 0.6901961,
            },
            Color {
                r: 0.6274510,
                g: 0.4392157,
                b: 0.8000000,
            },
            Color {
                r: 0.4862745,
                g: 0.4392157,
                b: 0.8156863,
            },
            Color {
                r: 0.4078431,
                g: 0.4549020,
                b: 0.8156863,
            },
            Color {
                r: 0.4078431,
                g: 0.5333334,
                b: 0.8000000,
            },
            Color {
                r: 0.4078431,
                g: 0.6117647,
                b: 0.7529412,
            },
            Color {
                r: 0.4078431,
                g: 0.7058824,
                b: 0.5803922,
            },
            Color {
                r: 0.4549020,
                g: 0.7058824,
                b: 0.4549020,
            },
            Color {
                r: 0.5176471,
                g: 0.7058824,
                b: 0.4078431,
            },
            Color {
                r: 0.6117647,
                g: 0.6588235,
                b: 0.3921569,
            },
            // Light
            Color {
                r: 0.8156863,
                g: 0.7058824,
                b: 0.4235294,
            },
            Color {
                r: 0.8627451,
                g: 0.8627451,
                b: 0.8627451,
            },
            Color {
                r: 0.9098039,
                g: 0.9098039,
                b: 0.3607843,
            },
            Color {
                r: 0.8627451,
                g: 0.7058824,
                b: 0.4078431,
            },
            Color {
                r: 0.9254902,
                g: 0.6588235,
                b: 0.5019608,
            },
            Color {
                r: 0.9254902,
                g: 0.6274510,
                b: 0.6274510,
            },
            Color {
                r: 0.8627451,
                g: 0.6117647,
                b: 0.8156863,
            },
            Color {
                r: 0.7686275,
                g: 0.6117647,
                b: 0.9254902,
            },
            Color {
                r: 0.6588235,
                g: 0.6274510,
                b: 0.9254902,
            },
            Color {
                r: 0.5647059,
                g: 0.6431373,
                b: 0.9254902,
            },
            Color {
                r: 0.5647059,
                g: 0.7058824,
                b: 0.9254902,
            },
            Color {
                r: 0.5647059,
                g: 0.8000000,
                b: 0.9098039,
            },
            Color {
                r: 0.5647059,
                g: 0.8941177,
                b: 0.7529412,
            },
            Color {
                r: 0.6431373,
                g: 0.8941177,
                b: 0.6431373,
            },
            Color {
                r: 0.7058824,
                g: 0.8941177,
                b: 0.5647059,
            },
            Color {
                r: 0.8000000,
                g: 0.8313726,
                b: 0.5333334,
            },
            // Dark
            Color {
                r: 0.7215686,
                g: 0.6117647,
                b: 0.3450980,
            },
            Color {
                r: 0.7843137,
                g: 0.7843137,
                b: 0.7843137,
            },
            Color {
                r: 0.8156863,
                g: 0.8156863,
                b: 0.3137255,
            },
            Color {
                r: 0.8000000,
                g: 0.6274510,
                b: 0.3607843,
            },
            Color {
                r: 0.8784314,
                g: 0.5803922,
                b: 0.4392157,
            },
            Color {
                r: 0.8784314,
                g: 0.5333334,
                b: 0.5333334,
            },
            Color {
                r: 0.8156863,
                g: 0.5176471,
                b: 0.7529412,
            },
            Color {
                r: 0.7058824,
                g: 0.5176471,
                b: 0.8627451,
            },
            Color {
                r: 0.5803922,
                g: 0.5333334,
                b: 0.8784314,
            },
            Color {
                r: 0.4862745,
                g: 0.5490196,
                b: 0.8784314,
            },
            Color {
                r: 0.4862745,
                g: 0.6117647,
                b: 0.8627451,
            },
            Color {
                r: 0.4862745,
                g: 0.7058824,
                b: 0.8313726,
            },
            Color {
                r: 0.4862745,
                g: 0.8156863,
                b: 0.6745098,
            },
            Color {
                r: 0.5490196,
                g: 0.8156863,
                b: 0.5490196,
            },
            Color {
                r: 0.6117647,
                g: 0.8000000,
                b: 0.4862745,
            },
            Color {
                r: 0.7058824,
                g: 0.7529412,
                b: 0.4705882,
            },
            // Lightest
            Color {
                r: 0.9098039,
                g: 0.8000000,
                b: 0.4862745,
            },
            Color {
                r: 0.9254902,
                g: 0.9254902,
                b: 0.9254902,
            },
            Color {
                r: 0.9882353,
                g: 0.9882353,
                b: 0.4078431,
            },
            Color {
                r: 0.9254902,
                g: 0.7843137,
                b: 0.4705882,
            },
            Color {
                r: 0.9882353,
                g: 0.7372549,
                b: 0.5803922,
            },
            Color {
                r: 0.9882353,
                g: 0.7058824,
                b: 0.7058824,
            },
            Color {
                r: 0.9254902,
                g: 0.6901961,
                b: 0.8784314,
            },
            Color {
                r: 0.8313726,
                g: 0.6901961,
                b: 0.9882353,
            },
            Color {
                r: 0.7372549,
                g: 0.7058824,
                b: 0.9882353,
            },
            Color {
                r: 0.6431373,
                g: 0.7215686,
                b: 0.9882353,
            },
            Color {
                r: 0.6431373,
                g: 0.7843137,
                b: 0.9882353,
            },
            Color {
                r: 0.6431373,
                g: 0.8784314,
                b: 0.9882353,
            },
            Color {
                r: 0.6431373,
                g: 0.9882353,
                b: 0.8313726,
            },
            Color {
                r: 0.7215686,
                g: 0.9882353,
                b: 0.7215686,
            },
            Color {
                r: 0.7843137,
                g: 0.9882353,
                b: 0.6431373,
            },
            Color {
                r: 0.8784314,
                g: 0.9254902,
                b: 0.6117647,
            },
            Color {
                r: 0.9882353,
                g: 0.8784314,
                b: 0.5490196,
            },
        ];

        Self {
            index: 5, // Because I like the color
            colors,
        }
    }
    fn pop_color(&mut self) -> Color {
        let color = self.colors.remove(self.index);
        self.index = (self.index + 1) % self.colors.len();
        color
    }
    fn push_color(&mut self, color: Color) {
        self.colors.push(color)
    }
}

// Returns whether or not the player was actually there to be removed
fn remove_player(
    id: u8,
    player_states: &mut HashMap<u8, PlayerState>,
    color_picker: &mut ColorPicker,
    forced: bool,
) -> bool {
    let mut msg = format!(
        "Player {} {}",
        id,
        if forced { "kicked for idling" } else { "left" }
    );
    if let Some(player_state) = player_states.remove(&id) {
        msg.push_str(&format!(
            ", name: {}, color: {:?}",
            player_state.name, player_state.color
        ));
        color_picker.push_color(player_state.color);
        println!("{}", msg);
        return true;
    }
    false
}

#[allow(clippy::never_loop)]
fn process_game_control_requests(
    game_control_server_socket: &mut zmq::Socket,
    game_settings: &mut GameSettings,
    player_states: &mut HashMap<u8, PlayerState>,
    rng: &mut ThreadRng,
    color_picker: &mut ColorPicker,
    high_scores: &mut HighScores,
) {
    'gamecontrol: loop {
        match game_control_server_socket.recv_multipart(0) {
            Err(_e) => break 'gamecontrol,
            Ok(multipart_message) => {
                let return_identity = &multipart_message[0];
                let msg: GameControlMsg = deserialize(&multipart_message[2][..]).unwrap();
                match msg {
                    GameControlMsg::Join { name } => {
                        let result: Result<u8, String>;
                        loop {
                            // Is the game full?
                            if player_states.len() >= game_settings.max_players as usize {
                                let err = format!(
                                    "Join Failed: No room for player {} - {} players is the max!",
                                    name, game_settings.max_players
                                );
                                println!("{}", err);
                                result = Err(err);
                                break;
                            }
                            // Is the name already taken?
                            if player_states
                                .values()
                                .map(|player_state| &player_state.name)
                                .any(|x| x == &name)
                            {
                                let err =
                                    format!("Join Failed: Name \"{}\" is already taken.", name);
                                println!("{}", err);
                                result = Err(err);
                                break;
                            }
                            // Find a random, unused, non-zero id
                            let mut id;
                            loop {
                                id = rng.gen::<u8>();
                                if (id != 0) && !player_states.contains_key(&id) {
                                    break;
                                }
                            }
                            // Assign player a color
                            let color = color_picker.pop_color();
                            // Create the new player state
                            let player_state = PlayerState::new(
                                &game_settings,
                                id,
                                name.clone(),
                                color,
                                new_in_square(0.6, rng),
                                0.05,
                            );
                            high_scores.add_player(&player_state.name);
                            player_states.insert(id, player_state);
                            println!("Joined: {} (id {})", name, id,);
                            result = Ok(id);
                            break;
                        }
                        game_control_server_socket
                            .send_multipart(
                                &[&return_identity[..], &[], &serialize(&result).unwrap()],
                                0,
                            )
                            .unwrap();
                    }
                    GameControlMsg::Leave { id } => {
                        let succeeded = remove_player(id, player_states, color_picker, false);
                        if succeeded {
                            println!("Player {} left voluntarily.", id);
                        }
                        game_control_server_socket
                            .send_multipart(
                                &[&return_identity[..], &[], &serialize(&succeeded).unwrap()],
                                0,
                            )
                            .unwrap();
                    }
                    GameControlMsg::Fetch => {
                        game_control_server_socket
                            .send_multipart(
                                &[
                                    &return_identity[..],
                                    &[],
                                    &serialize(&game_settings).unwrap(),
                                ],
                                0,
                            )
                            .unwrap();
                        println!("A player fetches new settings.");
                    }
                }
            }
        }
    }
}

fn coalesce_player_input(
    player_input_server_socket: &mut zmq::Socket,
    player_states: &mut HashMap<u8, PlayerState>,
    player_inputs: &mut HashMap<u8, PlayerInput>,
) {
    while let Ok(bytes) = player_input_server_socket.recv_bytes(0) {
        let player_input: PlayerInput = deserialize(&bytes[..]).unwrap();
        if let Some(player_state) = player_states.get_mut(&player_input.id) {
            player_state.drop_timer.reset();
        }
        player_inputs
            .entry(player_input.id)
            .or_insert_with(|| player_input.clone())
            .coalesce(player_input);
    }
}

fn update_state(
    player_states: &mut HashMap<u8, PlayerState>,
    player_inputs: &mut HashMap<u8, PlayerInput>,
    game_settings: &mut GameSettings,
    color_picker: &mut ColorPicker,
    delta: Duration,
    rng: &mut ThreadRng,
    high_scores: &mut HighScores,
) {
    let delta_f32 = delta.f32();

    // Update player timers, spawn anyone who is ready
    // See if any players disconnect, die, or spawn
    for (id, player_state) in player_states.iter_mut() {
        // First update any delta-dependent state
        player_state.update(delta);
        // Anyone ready to spawn?
        if player_state.dead && player_state.respawn_timer.ready {
            player_state.respawn(new_in_square(0.9, rng), &format!("Player {} spawns", id));
        }
    }

    // Process input to affect velocities
    for (id, player_input) in &mut player_inputs.iter() {
        if let Some(player_state) = player_states.get_mut(id) {
            // Ignore input from dead players, and remove their movement.
            if player_state.dead {
                player_state.velocity = Vec2::zeros();
                continue;
            }
            // Instantaneously face a direction
            player_state.direction = player_input.direction;
            // Update current velocity
            let clamped_move_amount = if player_input.move_amount.magnitude() > 1.0 {
                player_input.move_amount.normalize()
            } else {
                player_input.move_amount
            };
            if clamped_move_amount.magnitude() > game_settings.move_threshold {
                // Player is moving -- add input to current velocity
                player_state.velocity = player_state.velocity
                    + (clamped_move_amount * game_settings.acceleration * delta_f32);
            } else {
                // Player is holding still, apply drag to current velocity
                player_state.velocity =
                    player_state.velocity * (1.0 - (game_settings.drag * delta_f32));
            }
            // If the player is attacking, then he can only go half as fast
            if player_input.attack {
                // player_state.velocity = player_state
                //     .velocity
                //     .clamped_to(game_settings.max_velocity * 0.5);
                clamp_vec_to_magnitude(&mut player_state.velocity, game_settings.max_velocity * 0.5)
            } else {
                clamp_vec_to_magnitude(&mut player_state.velocity, game_settings.max_velocity);
            }
        }
    }
    // Process all the velocities to affect position (not just the players who had input this loop)
    for (id, player_state) in player_states.iter_mut() {
        // Dead players don't move
        if player_state.dead {
            continue;
        }
        // Apply velocity to position
        player_state.pos = player_state.pos + (player_state.velocity * delta_f32);
        // Don't go all the way into the dark!
        let boundary = 1.0;
        if player_state.pos.x < -boundary
            || player_state.pos.x > boundary
            || player_state.pos.y < -boundary
            || player_state.pos.y > boundary
        {
            high_scores.penalize(&player_state.name);
            player_state.die(&format!(
                "Player {} was eaten by a grue.  Should have stayed in the light!",
                id
            ));
        }
    }

    // Get everyone who wants to attack
    let mut attacking_ids: Vec<u8> = vec![];
    for (id, player_input) in player_inputs.iter_mut() {
        // first we need to figure out who is trying to attack, and turn off their sticky attack bool
        if player_input.attack {
            attacking_ids.push(*id);
            player_input.attack = false;
        }
    }
    // Try to attack
    for id in attacking_ids {
        let mut attacker: PlayerState;
        if let Some(maybe_attacker) = player_states.remove(&id) {
            attacker = maybe_attacker;
        } else {
            // ZeroMQ lets clients that were connected to a previous server keep sending input. /facepalm
            continue;
        }
        // Dead players don't attack
        if attacker.dead {
            player_states.insert(id, attacker);
            continue;
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
            if defender.dead {
                continue;
            }
            if distance(&attacker.pos, &defender.pos) <= attacker.weapon.radius + attacker.radius {
                missed = false;
                if (defender.health > 0.0) && ((defender.health - attacker.weapon.damage) <= 0.0) {
                    high_scores.score(&attacker.name);
                }
                defender.health -= attacker.weapon.damage;
                attacker
                    .player_events
                    .push(PlayerEvent::AttackHit { id: defender_id });
                defender.player_events.push(PlayerEvent::TookDamage);
                println!(
                    "Player {} swings and hits ({}) for {:2.1} damage bringing him to {} health.",
                    id, defender_id, attacker.weapon.damage, defender.health
                );
            }
        }
        if missed {
            attacker.player_events.push(PlayerEvent::AttackMiss);
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
        if !player_state.dead && player_state.health <= 0.0 {
            player_state.die(&format!("Player {} dies from his wounds.", id));
        }
        player_states.insert(id, player_state);
    }
}

fn main() {
    let ctx = zmq::Context::new();

    let mut game_control_server_socket = ctx.socket(zmq::ROUTER).unwrap();
    game_control_server_socket.set_rcvtimeo(0).unwrap();
    game_control_server_socket
        .bind(&format!("tcp://*:{}", net::GAME_CONTROL_PORT))
        .unwrap();

    let game_state_server_socket = ctx.socket(zmq::PUB).unwrap();
    game_state_server_socket
        .bind(&format!("tcp://*:{}", net::GAME_STATE_PORT))
        .unwrap();

    let mut player_input_server_socket = ctx.socket(zmq::PULL).unwrap();
    player_input_server_socket.set_rcvtimeo(0).unwrap();
    player_input_server_socket
        .bind(&format!("tcp://*:{}", net::PLAYER_INPUT_PORT))
        .unwrap();

    let mut loop_iterations: i64 = 0;
    let mut loop_start = Instant::now();
    let mut frame_timer = timer::Timer::from_nanos(16_666_666); // 60 FPS
    let mut color_picker = ColorPicker::new();
    let mut game_settings = GameSettings::new();
    let mut rng = thread_rng();
    let mut frame_number: u64 = 0;
    let mut player_states = HashMap::<u8, PlayerState>::new();
    let mut player_inputs = HashMap::<u8, PlayerInput>::new();
    let mut high_scores = HighScores::new();
    let sleep_delay = Duration::from_millis(1);

    println!("--------------------------------------------------------------");
    println!("Server started (Ctrl-C to stop)\n{:#?}", game_settings);
    loop {
        let delta = loop_start.elapsed();
        loop_start = Instant::now();
        frame_timer.update(delta);
        // Sleep just a bit to avoid a busy-loop from sucking up 100% of a CPU
        if delta < sleep_delay {
            thread::sleep(Duration::from_micros(50));
        }
        loop_iterations += 1;

        // Handle and reply to all Game Control requests. The game settings might get changed.
        process_game_control_requests(
            &mut game_control_server_socket,
            &mut game_settings,
            &mut player_states,
            &mut rng,
            &mut color_picker,
            &mut high_scores,
        );

        // Handle and coalesce all the player input we've received so far into player_inputs
        coalesce_player_input(
            &mut player_input_server_socket,
            &mut player_states,
            &mut player_inputs,
        );

        // Move, attack, etc.
        update_state(
            &mut player_states,
            &mut player_inputs,
            &mut game_settings,
            &mut color_picker,
            delta,
            &mut rng,
            &mut high_scores,
        );

        // Process a frame (if it's time)

        if frame_timer.ready {
            frame_timer.reset();

            let top10 = high_scores.top10();
            // Broadcast new game state computed this frame
            if frame_number % 1800 == 0 {
                let status = format!(
                    "STATUS: Frame: {}, Loops during latest frame: {}\n{}",
                    frame_number, loop_iterations, top10
                );
                println!("{}", status);
            }
            let game_state = GameState {
                frame_number,
                delta,
                game_settings_hash: game_settings.get_hash(),
                player_states: player_states.clone(),
                high_scores: top10,
            };
            game_state_server_socket
                .send(&serialize(&game_state).unwrap(), 0)
                .unwrap();
            for player_state in player_states.values_mut() {
                player_state.new_frame();
            }
            loop_iterations = 0;
            frame_number += 1;
        }
    }
}
