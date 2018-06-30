#[macro_use]
extern crate glium;

pub mod gfx;
pub mod net;
pub mod timer;

use timer::Timer;

use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::Duration;

extern crate zmq;
#[macro_use]
extern crate serde_derive;
extern crate bincode;

/// Represents (x, y) coordinates in OpenGL space that fill your window.  The OpenGL window is
/// (-1.0, -1.0) in the bottom left to (1.0, 1.0) in the top right.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Position {
    pub x : f32,
    pub y : f32,
}

impl Position {
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    pub fn distance_between(&self, pos : Position) -> f32 {
        ((self.x - pos.x).powi(2) + (self.y - pos.y).powi(2)).sqrt()
    }
}

/// Convenience trait that adds an `.f32()` method that returns a 32-bit float representation of
/// something.  Implemented for std::time::Duration and rusty_sword_arena::timer::Timer.
pub trait Floatable {
    fn f32(&self) -> f32;
}

impl Floatable for Duration {
    fn f32(&self) -> f32 {
        self.as_secs() as f32 + self.subsec_nanos() as f32 * 1e-9
    }
}

/// Abstracted button values you may receive (arrow keys and WASD keys combined into directions, for example)
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ButtonValue {
    /// Arrow Up, W, Comma (Dvorak)
    Up,
    /// Arrow Down, S, O (Dvorak)
    Down,
    /// Arrow Left, A
    Left,
    /// Arrow Right, D, E (Dvorak)
    Right,
    /// Left Mouse Button, Space Bar, Backspace (Kinesis Advantage Keyboard)
    Attack,
    /// Escape
    Quit,
}

/// Whether a button was pressed or released
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ButtonState {
    Pressed,
    Released,
}

/// Client Events that may occur
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum Event {
    /// The window was closed somehow, so we better quit
    WindowClosed,
    /// The mouse is now at this location (OpenGL coordinates - can extend past what's viewable if
    /// the mouse is outside the window)
    MouseMoved { position : Position },
    Button {
        button_value : ButtonValue,
        button_state : ButtonState
    },
}

/// Various game control actions. Join a game, leave a game, just fetch updated game settings.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum GameControlMsg {
    Join  { name : String },
    Leave { id : u8 },
    Fetch { id : u8 },
}

/// Angle denotes a direction something is facing, in radians.
pub type Angle = f32;

/// A color with 32-bit float parts from `[0.0, 1.0]` suitable for OpenGL.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Color {
    /// Red
    pub r : f32,
    /// Green
    pub g : f32,
    /// Blue
    pub b : f32,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
/// Player settings that only change when someone joins/leaves the game, as opposed to every frame.
pub struct PlayerSetting {
    pub name : String,
    pub color_name : String,
    pub color : Color,
}

/// Server returns a GameSetting in response to receiving a PlayerSync
/// Game settings and player names and colors (including your own) are all in there.  You will
/// need to re-parse this every time someone joins or leaves the game.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GameSetting {
    /// The ID of your player.
    pub your_player_id : u8,
    /// The maximum amount of players this server will allow
    pub max_players : u8,
    /// OpenGL units. Collision radius of players (size of a player)
    pub player_radius : f32,
    /// How fast (in OpenGL units) a player moves per second
    pub move_speed : f32,
    /// Percentage `[0.0, 1.0]`.  How much the server will dampen your player's movement if moving
    /// exactly backwards.  Dampening effect is zero when moving exactly forwards, and linearly
    /// scales in movement directions between straight forward and straight backward.
    pub move_dampening : f32,
    /// Seconds. How long the server will wait to respawn a player who dies.
    pub respawn_delay : f32,
    /// Milliseconds. How long the server will allow not receiving input before dropping a player.
    pub drop_time: u64,
    /// Map of player id to settings such as names and colors. Note that this includes your own name
    /// _which may not be what you requested originally!_.
    pub player_settings : HashMap<u8, PlayerSetting>,
}

impl GameSetting {
    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Hash for GameSetting {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.your_player_id.hash(state);
        self.max_players.hash(state);
        (self.player_radius as u32).hash(state);
        (self.move_speed as u32).hash(state);
        (self.move_dampening as u32).hash(state);
        (self.respawn_delay as u32).hash(state);
        self.drop_time.hash(state);
        // PlayerSetting entries are assumed to be immutable, so we'll only look at the keys
        let mut sorted_keys : Vec<u8> = self.player_settings.keys().map(|x| {*x}).collect();
        sorted_keys.sort();
        sorted_keys.hash(state);
    }
}

/// An event that has happened to your player this frame!  Note that it's possible to receive a
/// whole bunch of events in the same frame.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum PlayerEvent {
    /// Player has attacked and hit player id.
    AttackHit { id : u8 },
    /// Player has attacked, but not hit anyone.
    AttackMiss,
    /// Player has changed to a new weapon
    ChangeWeapon,
    /// Player has died
    Die,
    /// Player has stopped healing
    HealEnd,
    /// Player has started healing
    HealStart,
    /// Player has stopped moving
    MoveEnd,
    /// Player has started moving
    MoveStart,
    /// Player has spawned
    Spawn,
    /// Player has received damage
    TookDamage,
}

/// A weapon a player may hold
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Weapon {
    /// Something like "Rusty Sword", "Shiny Sword", "Rusty Spear", etc.
    pub description : String,
    /// How much damage the weapon can cause
    pub damage : f32,
    /// How long between attacks
    pub delay : f32,
    /// How far attacks reach from your player
    pub radius : f32,
}

impl Weapon {
    pub fn new() -> Self {
        Self {
            description : "Rusty Sword".to_string(),
            damage : 10.0,
            delay : 1.0,
            radius : 0.1,
        }
    }
}

/// The state of a player on the server. The server broadcasts these to all clients every frame as
/// part of a FrameState.  Note that you can receive `PlayerState`s before you have gotten a
/// corresponding GameSetting telling you their name and color!
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PlayerState {
    /// The ID of the player
    pub id : u8,
    /// The position of the player in OpenGL units.
    pub pos : Position,
    /// The angle of your player, aka the player is facing
    pub angle : Angle,
    /// Current velocity of the player
    pub velocity : f32,
    /// Current health of the player [0.0, 100.0]
    pub health : f32,
    /// Current regen rate of the player
    pub regen : f32,
    /// Current weapon of the player
    pub weapon : Weapon,
    /// Any player events that have occurred to the player this frame
    pub player_events : Vec<PlayerEvent>,
    /// What the server considers the player's horizontal axis input. Note this can be quite
    /// different from the input the player sent to the server.
    pub horiz_axis : f32,
    /// What the server considers the player's vertical axis input. Note this can be quite
    /// different from the input the player sent to the server.
    pub vert_axis : f32,
    /// How long until the player can attack again
    pub attack_timer : Timer,
    /// How long the server will wait to get input from you before disconnecting you
    pub drop_timer: Timer,
}

impl PlayerState {
    pub fn new(drop_time : u64) -> Self {
        Self {
            id : 0,
            pos : Position { x: 0.0, y: 0.0 },
            angle : 0.0,
            velocity : 0.0,
            health : 100.0,
            regen : 0.0,
            weapon : Weapon::new(),
            player_events : Vec::<PlayerEvent>::new(),
            horiz_axis: 0.0,
            vert_axis : 0.0,
            attack_timer : Timer::from_millis(500),
            drop_timer: Timer::from_millis(drop_time),
        }
    }
    pub fn update(&mut self, delta : Duration) {
        self.attack_timer.update(delta);
        self.drop_timer.update(delta);
    }
}

/// Once per frame, the server will broadcast a GameState to all clients.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GameState {
    /// Which frame we're on.  Starts at zero and increments by 1 each frame.
    pub frame_number : u64,
    /// The actual time the server measured since the previous frame.
    pub delta : Duration,
    /// The hash of the current game setting. Your client should store this somewhere. If it changes
    /// then something has changed (most likely a player has joined or disconnected), so you should
    /// send a GameControlMsg::Fetch to get the new GameSetting from the server and update your
    /// client state.
    pub game_setting_hash : u64,
    /// All of the player's states, including your own!
    pub player_states : HashMap<u8, PlayerState>,
}
/// Clients should send `PlayerInput`s to the server ASAP.  The quicker the server gets inputs, the
/// more accurate the simulation will be.  But of course, you also shouldn't overload the server
/// with too much traffic, because that's bad too.  Good rule of thumb: Coalesce 4 milliseconds
/// worth of input together, and send that.  That's about 4 times faster than frames are sent by the
/// server (60fps = 16.7ms).  The server should be able to handle ~250 pkts/sec per client.  I hope.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PlayerInput {
    /// The ID of your player
    pub id : u8,
    /// Whether you are attempting to attack (actual attack will occur if the server-side attack
    /// timer has reached 0)
    pub attack : bool,
    /// How much your player is attempting to move horizontally [-1.0, 1.0]. Positive is right.
    pub horiz_axis : f32,
    /// How much your player is attempting to move vertically [-1.0, 1.0]. Positive is up.
    pub vert_axis : f32,
    /// What angle your player is facing. You can turn instantly, you lucky dog.
    pub turn_angle : Angle,
}

impl PlayerInput {
    pub fn new() -> Self {
        Self {
            id : 0,
            attack : false,
            horiz_axis : 0.0,
            vert_axis : 0.0,
            turn_angle : 0.0,
        }
    }
    // Combine successive inputs into one input
    pub fn coalesce(&mut self, new : PlayerInput) {
        // Any attack sticks
        self.attack = self.attack || new.attack;
        // Anything else the new value wins
        self.horiz_axis = new.horiz_axis;
        self.vert_axis = new.vert_axis;
        self.turn_angle = new.turn_angle;
    }
}

