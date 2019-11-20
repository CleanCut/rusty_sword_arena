// THIS IS ONE REFERENCE IMPLEMENTATION
// IT IS NOT EXACTLY WHAT WE WILL CREATE DURING THE TUTORIAL...but it's pretty similar.

use rusty_sword_arena::{
    audio::Audio,
    game::{ButtonProcessor, GameEvent, PlayerEvent, PlayerInput, PlayerState, Vector2},
    gfx::{Img, Window},
    net::ConnectionToServer,
    timer::Timer,
    VERSION,
};
use std::collections::HashMap;
use std::env;
use std::f32::consts::PI;
use std::time::{Duration, Instant};

struct Player {
    player_state: PlayerState,
    player_img: Img,
    sword_img: Img,
    rip_img: Img,
    sword_timer: Timer,
}

impl Player {
    fn new(window: &Window, player_state: PlayerState) -> Self {
        let player_img = Img::new(
            window,
            player_state.pos,
            player_state.direction,
            Some(player_state.color),
            "media/player.png",
        );
        let sword_img = Img::new(
            window,
            player_state.pos,
            player_state.direction,
            None,
            "media/sword.png",
        );
        let rip_img = Img::new(
            window,
            player_state.pos,
            0.0,
            Some(player_state.color),
            "media/rip.png",
        );
        let mut sword_timer = Timer::from_millis(350);
        sword_timer.update(Duration::from_secs(5));
        Self {
            player_state,
            player_img,
            sword_img,
            rip_img,
            sword_timer,
        }
    }
    fn update_state(&mut self, player_state: PlayerState, audio: &mut Audio) {
        self.player_state = player_state;
        let ps = &mut self.player_state;
        self.player_img.pos = ps.pos;
        self.player_img.direction = ps.direction;
        self.sword_img.pos = ps.pos;
        self.rip_img.pos = ps.pos;
        // Process events for this player
        for player_event in ps.player_events.drain(..) {
            // Reset the sword timer
            match player_event {
                PlayerEvent::AttackHit { .. } => self.sword_timer.reset(),
                PlayerEvent::AttackMiss => self.sword_timer.reset(),
                _ => {}
            }
            // Play sounds
            match player_event {
                PlayerEvent::AttackMiss => audio.play("miss"),
                PlayerEvent::Die => audio.play("die"),
                PlayerEvent::Spawn => audio.play("spawn"),
                PlayerEvent::Join => audio.play("join"),
                PlayerEvent::TookDamage => audio.play("ow"),
                _ => (),
            }
        }
        // The timer being "ready" means the sword swing is over, so just point the sword forward
        if self.sword_timer.ready {
            self.sword_img.direction = ps.direction;
        } else {
            // If the timer is going, then put the sword in some portion of the swing animation
            self.sword_img.direction =
                ps.direction + (2.0 * PI * self.sword_timer.time_left_percent());
        }
    }
    fn update_timer(&mut self, dt: Duration) {
        self.sword_timer.update(dt);
    }
    fn draw(&self, window: &mut Window) {
        if self.player_state.dead {
            if !self.player_state.joining {
                window.draw(&self.rip_img);
            }
            return;
        }
        window.draw(&self.player_img);
        window.draw(&self.sword_img);
    }
}

fn main() {
    let mut args: Vec<String> = env::args().skip(1).collect();
    if args.len() != 2 {
        println!("Usage: (prog) name host");
        std::process::exit(2);
    }
    let host = args.pop().unwrap();
    let name = args.pop().unwrap();
    let mut connection = ConnectionToServer::new(&host);
    let response = connection.join(&name);
    if let Err(err) = response {
        println!("{}", err);
        std::process::exit(3);
    }
    let my_id = response.unwrap();
    let game_settings = connection.get_game_settings();

    println!(
        "Client v{} connected to server v{} at {}",
        VERSION, game_settings.version, host
    );

    let mut window = Window::new(None, "Rusty Sword Arena!");
    let mut players: HashMap<u8, Player> = HashMap::new();
    let mut mouse_pos = Vector2::new();
    let mut player_input = PlayerInput::with_id(my_id);
    let mut button_processor = ButtonProcessor::new();
    let mut instant = Instant::now();
    let mut dt = Duration::from_secs(0);

    let mut audio = Audio::new();
    audio.add("die", "media/die.ogg");
    audio.add("join", "media/join.ogg");
    audio.add("miss", "media/miss.ogg");
    audio.add("ow", "media/ow.ogg");
    audio.add("spawn", "media/spawn.ogg");
    audio.add("startup", "media/startup.ogg");

    'gameloop: loop {
        // Accumulate & send player input
        for event in window.poll_game_events() {
            match event {
                GameEvent::Quit => break 'gameloop,
                GameEvent::MouseMoved { position } => mouse_pos = position,
                GameEvent::Button {
                    button_state,
                    button_value,
                } => button_processor.process(button_state, button_value, &mut player_input),
            }
        }
        if let Some(my_player) = players.get(&my_id) {
            // If I know my position, I can set my direction to point towards the mouse
            player_input.direction = my_player.player_state.pos.angle_between(mouse_pos);
        }
        connection.send_player_input(&player_input);

        // Process any new game states
        for game_state in connection.poll_game_states() {
            // Remove players who no longer have a game state
            players.retain(|k, _| game_state.player_states.contains_key(k));
            // Create new players and update existing players
            for (id, player_state) in game_state.player_states {
                players
                    .entry(id)
                    .or_insert_with(|| Player::new(&window, player_state.clone()))
                    .update_state(player_state, &mut audio);
            }
        }

        // Update player timers
        for player in players.values_mut() {
            player.update_timer(dt);
        }

        // Draw a frame!
        window.drawstart();
        // Draw all the other players
        for (id, player) in &players {
            if *id == my_id {
                continue;
            }
            player.draw(&mut window);
        }
        // Draw my own player last, so I can always see myself
        if let Some(player) = players.get(&my_id) {
            player.draw(&mut window);
        }
        window.drawfinish();
        dt = instant.elapsed();
        instant = Instant::now();
    }

    println!("Leaving the game.");
    let succeeded = connection.leave(my_id);
    println!(
        "Server reports disconnection {}",
        if succeeded { "successful" } else { "failed" }
    );
}
