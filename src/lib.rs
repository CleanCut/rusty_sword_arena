#[macro_use]
extern crate glium;

pub mod net;
pub mod gfx;

use std::collections::HashMap;

extern crate zmq;
#[macro_use]
extern crate serde_derive;
extern crate bincode;

//use bincode::{serialize, deserialize};


/// Represents (x, y) coordinates in OpenGL space that fill your window.
#[derive(Copy, Clone, Debug)]
pub struct Position {
    pub x : f32,
    pub y : f32,
}

impl Position {
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
}



/// Abstracted key values you may receive (arrow keys and WASD keys combined, for example)
#[derive(Copy, Clone, Debug)]
pub enum KeyValue {
    Up,
    Down,
    Left,
    Right,
}

/// When KeyboardInput occurs, we want to know what state was entered
#[derive(Copy, Clone, Debug)]
pub enum KeyState {
    Pressed,
    Released,
}

/// Client Events that may occur
#[derive(Copy, Clone, Debug)]
pub enum Event {
    /// The window was closed somehow, so we better quit
    WindowClosed,
    /// The mouse is now at this location (OpenGL coordinates - can extend past what's viewable if
    /// the mouse is outside the window)
    MouseMoved { position : Position },
    KeyboardInput {
        key_value : KeyValue,
        key_state : KeyState
    },
}

/// Various game control actions. Join a game, leave a game, just fetch updated game settings.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum GameControlMsg {
    Join  { name : String },
    Leave { id : u8 },
    Fetch,
}

/// Angle denotes a direction something is facing, in radians.
pub type Angle = f32;

/// A color with 32-bit float parts from `[0.0, 1.0]` suitable for OpenGL.
#[derive(Copy, Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct Color {
    /// Red
    pub r : f32,
    /// Green
    pub g : f32,
    /// Blue
    pub b : f32,
}

/// Server returns a GameSettings in response to receiving a PlayerSync
/// Game settings and player names and colors (including your own) are all in there.  You will
/// need to re-parse this every time someone joins or leaves the game.
#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
pub struct GameSettings {
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
    /// Seconds. Server will never _send_ frame updates more frequently than this. When and how far
    /// apart they arrive is entirely up to the network.
    pub frame_delay : f32,
    /// Seconds. How long the server will wait to respawn a player who dies.
    pub respawn_delay : f32,
    /// Seconds. How long the server will allow not receiving input before dropping a player.
    pub drop_timeout : f32,
    /// Map of player id to names, including your own name _which may not be what you expect_.
    pub player_names : HashMap<u8, String>,
    /// Map of player id to player colors, including your own assigned color.
    pub player_colors : HashMap<u8, Color>,
}

pub enum PlayerEvent {
    Attack,
    ChangeWeapon,
    Die,
    HealEnd,
    HealStart,
    MoveEnd,
    MoveStart,
    Spawn,
    TookDamage,
}

pub struct Weapon {
    pub description : String,
    pub damage : f32,
    pub delay : f32,
    pub radius : f32,
}

pub struct PlayerState {
    pub id : u8,
    pub pos : Position,
    pub angle : Angle,
    pub velocity : f32,
    pub health : f32,
    pub regen : f32,
    pub weapon : Weapon,
    pub player_events : Vec<PlayerEvent>,
}
