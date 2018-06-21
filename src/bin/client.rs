extern crate rusty_sword_arena;
extern crate glium;

use rusty_sword_arena as rsa;
use rsa::{Color, Event, KeyState, KeyValue, GameControlMsg, Position};
use rsa::net::{ServerConnection};
use rsa::gfx::{angle_between, Display, Shape};

fn main() {

    let mut server_conn = ServerConnection::new("localhost");

    let msg = GameControlMsg::Join {name : "bob".to_string()};
    let game_settings = server_conn.game_control(msg).unwrap();
    let my_id = game_settings.your_player_id;
    println!("{:#?}", game_settings);

    let mut display = Display::new(1024, 1024, &game_settings);
    let mut circles = vec![
        Shape::new_circle(&display, game_settings.player_radius, Position::new(), 0.0, Color { r : 0.1, g : 0.2, b : 1.0 }),
        Shape::new_circle(&display, game_settings.player_radius, Position { x : 0.5, y : 0.5 }, 0.0, Color { r : 1.0, g : 0.1, b : 0.1 }),
    ];

    let mut horiz_axis = 0.0f32;
    let mut vert_axis = 0.0f32;
    let mut mouse_pos = Position { x : 0.0, y : 0.0 };
    'gameloop:
    loop {
        for event in display.events() {
            match event {
                Event::WindowClosed => break 'gameloop,
                Event::MouseMoved { position } => mouse_pos = position,
                Event::KeyboardInput { key_state, key_value } => {
                    let amount = match key_state {
                        KeyState::Pressed => { 1.0 },
                        KeyState::Released => { 0.0 },
                    };
                    match key_value {
                        KeyValue::Up    => vert_axis  = amount,
                        KeyValue::Down  => vert_axis  = -amount,
                        KeyValue::Left  => horiz_axis = -amount,
                        KeyValue::Right => horiz_axis = amount,
                    }
                },
            }
        }

        // Update position
        if circles.len() > 0 {
            circles[0].direction = angle_between(circles[0].pos, mouse_pos);
            circles[0].pos.x += game_settings.move_speed * horiz_axis;
            circles[0].pos.y += game_settings.move_speed * vert_axis;
        }
        display.draw(&circles);
    }

    println!("Disconnecting from server.");
    let msg = GameControlMsg::Leave { id : my_id };
    let game_settings = server_conn.game_control(msg).unwrap();
    println!("Final server settings: {:#?}", game_settings);

}
