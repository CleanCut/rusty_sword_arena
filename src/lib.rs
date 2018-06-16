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



/// Various game control actions. Join a game, leave a game, just fetch updated game settings.
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum GameControlMsg {
    Join  { name : String },
    Leave { name : String },
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
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct GameSettings {
    /// The ID of your player.
    pub your_player_id : u8,
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
