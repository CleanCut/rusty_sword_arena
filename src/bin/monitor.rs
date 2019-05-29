//! Like the client, only it just connects to a server and displays what's happening without
//! actually joining a game or sending any input.





use impose::Audio;
use rusty_sword_arena::game::{Color, InputEvent, PlayerEvent, PlayerState};
use rusty_sword_arena::gfx::{Shape, Window};
use rusty_sword_arena::net::ServerConnection;
//use rusty_sword_arena::timer::Timer;
use rusty_sword_arena::VERSION;
use std::collections::HashMap;
use std::env;

struct Player {
    player_state: PlayerState,
    body_shape: Shape,
    sword_shape: Shape,
}

impl Player {
    fn new(window: &Window, player_state: PlayerState) -> Self {
        let body_shape = Shape::new_circle(
            window,
            player_state.radius,
            player_state.pos,
            player_state.direction,
            player_state.color,
        );
        let sword_shape = Shape::new_ring(
            window,
            player_state.weapon.radius,
            player_state.pos,
            player_state.direction,
            Color::new(1.0, 0.0, 0.0),
        );
        Self {
            player_state,
            body_shape,
            sword_shape,
        }
    }
    fn update_state(&mut self, player_state: PlayerState) {
        self.body_shape.pos = player_state.pos;
        self.body_shape.direction = player_state.direction;
        self.sword_shape.pos = player_state.pos;
        self.sword_shape.direction = player_state.direction;
        self.player_state = player_state;
    }
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 1 {
        println!("Usage: (prog) host");
        std::process::exit(2);
    }
    let host = args.pop().unwrap();
    let mut server_conn = ServerConnection::new(&host);
    let game_setting = server_conn.get_game_setting();

    println!(
        "Monitor v{} connected to server v{} at {}",
        VERSION, game_setting.version, host
    );

    let mut window = Window::new(None);
    let mut players = HashMap::<u8, Player>::new();
    let mut audio = Audio::new();
    audio.add_audio("miss", "media/miss.ogg");
    audio.add_audio("change_weapon", "media/change_weapon.ogg");
    audio.add_audio("die", "media/die.ogg");
    audio.add_audio("spawn", "media/spawn.ogg");
    audio.add_audio("join", "media/join.ogg");
    audio.add_audio("leave", "media/leave.ogg");
    audio.add_audio("ow", "media/ow.ogg");

    'gameloop: loop {
        // Accumulate user input into one struct
        for event in window.poll_input_events() {
            match event {
                InputEvent::WindowClosed => break 'gameloop,
                _ => (),
            }
        }

        // Process any new game states
        let new_game_states = server_conn.poll_game_states();
        for game_state in new_game_states {
            // Remove any players who are no longer in the game
            players.retain(|k, _v| game_state.player_states.contains_key(k));
            // Update or add all players that have states
            for (id, player_state) in game_state.player_states {
                if players.contains_key(&id) {
                    players.get_mut(&id).unwrap().update_state(player_state);
                } else {
                    players.insert(id, Player::new(&window, player_state));
                }
            }
        }
        // Process Player Events
        for (_id, player) in &mut players {
            for player_event in &mut player.player_state.player_events {
                match player_event {
                    PlayerEvent::AttackMiss => audio.play("miss"),
                    PlayerEvent::Die => audio.play("die"),
                    PlayerEvent::Spawn => audio.play("spawn"),
                    PlayerEvent::Join => audio.play("join"),
                    PlayerEvent::Leave => audio.play("leave"),
                    PlayerEvent::TookDamage => audio.play("ow"),
                    PlayerEvent::ChangeWeapon => audio.play("change_weapon"),
                    _ => (),
                }
            }
        }

        // Draw a frame!
        window.drawstart();
        // Draw all the bodies
        for (_id, player) in &players {
            if player.player_state.dead {
                continue;
            }
            window.draw(&player.body_shape);
        }
        // Draw all the swords
        for (_id, player) in &players {
            if player.player_state.dead {
                continue;
            }
            window.draw(&player.sword_shape);
        }
        window.drawfinish();
    }

    println!("Shutting down gracefully.");
}
