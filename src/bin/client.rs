extern crate rusty_sword_arena;
extern crate glium;

use rusty_sword_arena as rsa;
use rsa::{Color, Event, ButtonState, ButtonValue, GameControlMsg, PlayerInput, PlayerState, Position};
use rsa::net::{ServerConnection};
use rsa::gfx::{angle_between, Display, Shape};
use std::thread;
use std::time::Duration;

fn main() {

    let mut server_conn = ServerConnection::new("localhost");

    let msg = GameControlMsg::Join {name : "bob".to_string()};
    let game_settings = server_conn.send_game_control(msg).unwrap();
    let my_id = game_settings.your_player_id;
    println!("{:#?}", game_settings);

    let mut display = Display::new(1024, 1024, &game_settings);
    let mut circles = vec![
        Shape::new_circle(&display, game_settings.player_radius, Position::new(), 0.0, Color { r : 0.1, g : 0.2, b : 1.0 }),
        Shape::new_circle(&display, game_settings.player_radius, Position { x : 0.5, y : 0.5 }, 0.0, Color { r : 1.0, g : 0.1, b : 0.1 }),
    ];

    let mut attack = false;
    let mut horiz_axis = 0.0f32;
    let mut vert_axis = 0.0f32;
    let mut mouse_pos = Position { x : 0.0, y : 0.0 };
    let mut my_state = PlayerState::new();
    'gameloop:
    loop {
        // Process user input and send it to the server
        let mut new_input = false;
        for event in display.events() {
            new_input = true;
            match event {
                Event::WindowClosed => break 'gameloop,
                Event::MouseMoved { position } => mouse_pos = position,
                Event::Button { button_state, button_value } => {
                    let axis_amount = match button_state {
                        ButtonState::Pressed => { 1.0 },
                        ButtonState::Released => { 0.0 },
                    };
                    match button_value {
                        ButtonValue::Up    => vert_axis  = axis_amount,
                        ButtonValue::Down  => vert_axis  = -axis_amount,
                        ButtonValue::Left  => horiz_axis = -axis_amount,
                        ButtonValue::Right => horiz_axis = axis_amount,
                        ButtonValue::Attack => attack = match button_state {
                            ButtonState::Pressed => { true },
                            ButtonState::Released => { false }
                        },
                        ButtonValue::Quit => break 'gameloop,
                    }
                },
            }
        }
        if new_input {
            server_conn.send_player_input(PlayerInput {
                id : my_id,
                attack,
                horiz_axis,
                vert_axis,
                turn_angle : angle_between(my_state.pos, mouse_pos),
            });
        }

        // See if there are new game states to process
        let new_game_states = server_conn.recv_game_states();
        if !new_game_states.is_empty() {
            for game_state in new_game_states {
                println!("{:#?}", game_state);
            }

        }
        display.draw(&Vec::<Shape>::new());
        // Update position
        //if circles.len() > 0 {
        //    circles[0].direction = angle_between(circles[0].pos, mouse_pos);
        //    circles[0].pos.x += game_settings.move_speed * horiz_axis;
        //    circles[0].pos.y += game_settings.move_speed * vert_axis;
        //}
        thread::sleep(Duration::from_millis(1));
    }

    println!("Disconnecting from server.");
    let msg = GameControlMsg::Leave { id : my_id };
    let game_settings = server_conn.send_game_control(msg).unwrap();
    println!("Final server settings: {:#?}", game_settings);

}
