extern crate rusty_sword_arena;
extern crate glium;

use rusty_sword_arena as rsa;
use rsa::{GameControlMsg};
use rsa::net::{ServerConnection};
use rsa::gfx::Display;

fn main() {

    let mut server_conn = ServerConnection::new("localhost");

    let msg = GameControlMsg::Join {name : "bob".to_string()};
    let game_settings = server_conn.game_control(msg).unwrap();

    let mut display = Display::new(1024, 1024, game_settings);
    loop {
        display.update();
        display.draw();
    }


}
