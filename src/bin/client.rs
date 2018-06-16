extern crate rusty_sword_arena;
extern crate glium;

use rusty_sword_arena as rsa;
use rsa::{GameControlMsg};
use rsa::net::{ServerConnection};
use rsa::gfx::Display;

fn main() {

    let mut server_conn = ServerConnection::new("localhost");

    let msg = GameControlMsg::Join {name : "bob".to_string()};
    if let Ok(game_settings) = server_conn.game_control(msg) {
        println!("Got game settings! {:?}", game_settings);
    }

    let mut display = Display::new(1024, 1024);
    loop {
        display.update();
        display.draw();
    }


}
