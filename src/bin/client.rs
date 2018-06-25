extern crate rusty_sword_arena;
extern crate glium;

use rusty_sword_arena::{Color, Event, ButtonState, ButtonValue, GameControlMsg, PlayerInput, PlayerState, Position};
use rusty_sword_arena::net::{ServerConnection};
use rusty_sword_arena::gfx::{angle_between, Display, Shape};
use std::thread;
use std::time::{Duration, Instant};

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

    let mut mouse_pos = Position { x : 0.0, y : 0.0 };
    let mut my_state = PlayerState::new();
    let mut my_input = PlayerInput::new();
    let mut last_input_sent = Instant::now();
    'gameloop:
    loop {
        // Accumulate user input into one struct
        for event in display.events() {
            match event {
                Event::WindowClosed => break 'gameloop,
                Event::MouseMoved { position } => {
                    mouse_pos = position;
                    my_input.turn_angle = angle_between(my_state.pos, mouse_pos);
                },
                Event::Button { button_state, button_value } => {
                    let axis_amount = match button_state {
                        ButtonState::Pressed => { 1.0 },
                        ButtonState::Released => { 0.0 },
                    };
                    match button_value {
                        ButtonValue::Up    => my_input.vert_axis  = axis_amount,
                        ButtonValue::Down  => my_input.vert_axis  = -axis_amount,
                        ButtonValue::Left  => my_input.horiz_axis = -axis_amount,
                        ButtonValue::Right => my_input.horiz_axis = axis_amount,
                        ButtonValue::Attack => {
                            // We only turn ON attack here.  We turn it off when the input is sent
                            // to the server.
                            let attack = match button_state {
                                ButtonState::Pressed => { true },
                                ButtonState::Released => { false }
                            };
                            if attack {
                                my_input.attack = true;
                            }
                        },
                        ButtonValue::Quit => break 'gameloop,
                    }
                },
            }
        }
        // Every 4 milliseconds, send accumulated input and reset attack
        if last_input_sent.elapsed() > Duration::from_millis(4) {
            server_conn.send_player_input(my_input.clone());
            my_input.attack = false;
            last_input_sent = Instant::now();
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
