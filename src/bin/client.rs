extern crate impose;
extern crate glium;
extern crate rusty_sword_arena;

use impose::Audio;
use rusty_sword_arena::game::{
    ButtonState,
    ButtonValue,
    Event,
    GameControlMsg,
    PlayerInput,
    PlayerState,
    Vector2,
};
use rusty_sword_arena::net::{ServerConnection};
use rusty_sword_arena::gfx::{Window, Shape};
use rusty_sword_arena::VERSION;
use std::collections::HashMap;
use std::env;
use std::time::{Duration, Instant};

fn main() {
    let mut args : Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        println!("Usage: (prog) name host")
    }
    let host = args.pop().unwrap();
    let name = args.pop().unwrap();
    let mut server_conn = ServerConnection::new(&host);

    let msg = GameControlMsg::Join {name};
    let mut game_setting = server_conn.send_game_control(msg);
    let my_id = game_setting.your_player_id;
    println!("Client v{} connected to server v{} at {}", VERSION, game_setting.version, host);

    let mut display = Window::new(None);
    let mut circles = HashMap::<u8, Shape>::new();
    let mut player_states = HashMap::<u8, PlayerState>::new();

    let mut mouse_pos = Vector2 { x : 0.0, y : 0.0 };
    let mut my_input = PlayerInput::new();
    my_input.id = my_id;
    let mut last_input_sent = Instant::now();
    let mut game_setting_hash : u64 = 0;

    let mut audio = Audio::new();
    audio.add_audio("hit", "media/hit.ogg");
    audio.add_audio("change_weapon", "media/change_weapon.ogg");
    audio.add_audio("die", "media/die.ogg");
    audio.add_audio("spawn", "media/spawn.ogg");
    audio.add_audio("join", "media/join.ogg");
    audio.add_audio("leave", "media/leave.ogg");
    audio.add_audio("ow", "media/ow.ogg");
    audio.add_audio("startup", "media/startup.ogg");
    audio.play("startup");

    'gameloop:
    loop {
        // Accumulate user input into one struct
        for event in display.events() {
            match event {
                Event::WindowClosed => break 'gameloop,
                Event::MouseMoved { position } => {
                    mouse_pos = position;
                },
                Event::Button { button_state, button_value } => {
                    let axis_amount = match button_state {
                        ButtonState::Pressed => { 1.0 },
                        ButtonState::Released => { 0.0 },
                    };
                    match button_value {
                        ButtonValue::Up    => my_input.move_amount.y  = axis_amount,
                        ButtonValue::Down  => my_input.move_amount.y  = -axis_amount,
                        ButtonValue::Left  => my_input.move_amount.x = -axis_amount,
                        ButtonValue::Right => my_input.move_amount.x = axis_amount,
                        ButtonValue::Attack => {
                            my_input.attack = match button_state {
                                ButtonState::Pressed => { true },
                                ButtonState::Released => { false },
                            };
                        }
                        ButtonValue::Quit => break 'gameloop,
                    }
                },
            }
        }

        // Every 4 milliseconds, send accumulated input and reset attack
        if last_input_sent.elapsed() > Duration::from_millis(4) {
            if let Some(my_state) = player_states.get(&my_id) {
                my_input.direction = my_state.pos.angle_between(mouse_pos);
            }
            server_conn.send_player_input(my_input.clone());
            last_input_sent = Instant::now();
        }

        // Any new game states?
        let new_game_states = server_conn.recv_game_states();
        if !new_game_states.is_empty() {
            for mut game_state in new_game_states {
                if game_state.game_setting_hash != game_setting_hash {
                    let msg = GameControlMsg::Fetch { id : my_id };
                    game_setting = server_conn.send_game_control(msg);
                    game_setting_hash = game_state.game_setting_hash;
                    // Remove circles for any players who left
                    circles.retain(|k, _v| {game_setting.player_settings.contains_key(k)});
                }
                player_states.clear();
                player_states.extend(game_state.player_states.drain());
            }
        }
        // Update the circles
        for (id, player_state) in &player_states {
            // If a player is dead, try to remove his circle
            if player_state.dead {
                let _ = circles.remove(id);
                continue;
            }
            // Update existing circles for existing players
            if circles.contains_key(id) {
                let circle = circles.get_mut(id).unwrap();
                circle.direction = player_state.direction;
                circle.pos = player_state.pos;
            // Add new circles for new players
            } else {
                if let Some(player_setting) = game_setting.player_settings.get(id) {
                    circles.insert(
                        *id,
                        Shape::new_circle(
                            &display,
                            game_setting.player_radius,
                            player_state.pos,
                            player_state.direction,
                            player_setting.color));
                }
            }
        }

        display.drawstart();
        for circle in circles.values() {
            display.draw(&circle);
        }
        display.drawfinish();
    }

    println!("Disconnecting from server.");
    let msg = GameControlMsg::Leave { id : my_id };
    let _game_setting = server_conn.send_game_control(msg);
}
