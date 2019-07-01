// THIS IS ONE REFERENCE IMPLEMENTATION
// IT IS NOT EXACTLY WHAT WE WILL CREATE DURING THE TUTORIAL...but it's pretty similar.

use rusty_sword_arena::{
    audio::Audio,
    game::{ButtonState, ButtonValue, GameEvent, PlayerEvent, PlayerInput, PlayerState, Vector2},
    gfx::{Image, Shape, Window},
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
    body_shape: Shape,
    sword_img: Image,
    swing_timer: Timer,
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
        let sword_img = Image::new(
            window,
            player_state.pos,
            player_state.direction,
            "media/sword.png",
        );
        let mut sword_swing_timer = Timer::from_millis(350);
        sword_swing_timer.update(Duration::from_secs(5));
        Self {
            player_state,
            body_shape,
            sword_img,
            swing_timer: sword_swing_timer,
        }
    }
    fn update_state(&mut self, player_state: PlayerState) {
        self.body_shape.pos = player_state.pos;
        self.body_shape.direction = player_state.direction;
        self.sword_img.pos = player_state.pos;
        // Reset the swing timer (for animating the sword) if an attack was attempted
        for event in &player_state.player_events {
            match event {
                PlayerEvent::AttackHit { .. } => {
                    self.swing_timer.reset();
                }
                PlayerEvent::AttackMiss => {
                    self.swing_timer.reset();
                }
                _ => {}
            }
        }
        // The timer being "ready" means the swing is over, so just point the sword forward
        if self.swing_timer.ready {
            self.sword_img.direction = player_state.direction;
        } else {
            // If the timer is going, then put the sword in some portion of the swing animation
            self.sword_img.direction =
                self.player_state.direction + (2.0 * PI * self.swing_timer.time_left_percent());
        }
        self.player_state = player_state;
    }
    fn update(&mut self, dt: Duration, audio: &mut Audio) {
        self.swing_timer.update(dt);
        for player_event in self.player_state.player_events.drain(..) {
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
    fn draw(&self, window: &mut Window) {
        if self.player_state.dead {
            return;
        }
        window.draw_shape(&self.body_shape);
        window.draw_image(&self.sword_img);
    }
}

#[derive(Debug)]
struct MovementStack {
    horizontal: Vec<ButtonValue>,
    vertical: Vec<ButtonValue>,
}

impl MovementStack {
    fn new() -> Self {
        Self {
            horizontal: Vec::new(),
            vertical: Vec::new(),
        }
    }
    fn handle_buttons(
        &mut self,
        button_state: ButtonState,
        button_value: ButtonValue,
        player_input: &mut PlayerInput,
    ) {
        match button_state {
            ButtonState::Pressed => match button_value {
                ButtonValue::Up | ButtonValue::Down => self.vertical.push(button_value),
                ButtonValue::Left | ButtonValue::Right => self.horizontal.push(button_value),
                ButtonValue::Attack => player_input.attack = true,
            },
            ButtonState::Released => match button_value {
                ButtonValue::Up | ButtonValue::Down => self.vertical.retain(|&x| x != button_value),
                ButtonValue::Left | ButtonValue::Right => {
                    self.horizontal.retain(|&x| x != button_value)
                }
                ButtonValue::Attack => player_input.attack = false,
            },
        }
        // Set horizontal movement based on the stack
        if let Some(last_horiz) = self.horizontal.last() {
            match last_horiz {
                ButtonValue::Left => player_input.move_amount.x = -1.0,
                ButtonValue::Right => player_input.move_amount.x = 1.0,
                _ => {}
            }
        } else {
            player_input.move_amount.x = 0.0;
        }
        // Set vertical movement based on the stack
        if let Some(last_vert) = self.vertical.last() {
            match last_vert {
                ButtonValue::Up => player_input.move_amount.y = 1.0,
                ButtonValue::Down => player_input.move_amount.y = -1.0,
                _ => {}
            }
        } else {
            player_input.move_amount.y = 0.0;
        }
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
    let mut server_conn = ConnectionToServer::new(&host);
    let response = server_conn.join(&name);
    if let Err(err) = response {
        println!("{}", err);
        std::process::exit(3);
    }
    let my_id = response.unwrap();
    let game_setting = server_conn.get_game_setting();

    println!(
        "Client v{} connected to server v{} at {}",
        VERSION, game_setting.version, host
    );

    let mut window = Window::new(None, "Rusty Sword Arena!");
    let mut players = HashMap::<u8, Player>::new();

    let mut mouse_pos = Vector2 { x: 0.0, y: 0.0 };
    let mut player_input = PlayerInput::new();
    player_input.id = my_id;
    let mut movement_stack = MovementStack::new();
    let mut instant = Instant::now();
    let mut dt = Duration::from_secs(0);

    let mut audio = Audio::new();
    audio.add("miss", "media/miss.ogg");
    audio.add("change_weapon", "media/change_weapon.ogg");
    audio.add("die", "media/die.ogg");
    audio.add("spawn", "media/spawn.ogg");
    audio.add("join", "media/join.ogg");
    audio.add("leave", "media/leave.ogg");
    audio.add("ow", "media/ow.ogg");

    'gameloop: loop {
        // Accumulate user input into one struct
        for event in window.poll_game_events() {
            match event {
                GameEvent::Quit => break 'gameloop,
                GameEvent::MouseMoved { position } => {
                    mouse_pos = position;
                }
                GameEvent::Button {
                    button_state,
                    button_value,
                } => movement_stack.handle_buttons(button_state, button_value, &mut player_input),
            }
        }

        // Send player input
        if let Some(my_player) = players.get(&my_id) {
            player_input.direction = my_player.player_state.pos.angle_between(mouse_pos);
        }
        server_conn.send_player_input(&player_input);

        // Process any new game states
        let new_game_states = server_conn.poll_game_states();
        for game_state in new_game_states {
            // Remove any players who are no longer in the game
            players.retain(|k, _v| game_state.player_states.contains_key(k));
            // Update or add all players that have states
            for (id, player_state) in game_state.player_states {
                players
                    .entry(id)
                    .or_insert_with(|| Player::new(&window, player_state.clone()))
                    .update_state(player_state);
            }
        }
        // Process Player Events
        for player in players.values_mut() {
            player.update(dt, &mut audio);
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
    let succeeded = server_conn.leave(my_id);
    println!(
        "Server reports disconnection {}",
        if succeeded { "successful" } else { "failed" }
    );
}
