use super::timer::Timer;
use super::VERSION;

use rand::prelude::Rng;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::Add;
use std::ops::Mul;
use std::time::Duration;


/// 2D Vector (x, y) that can represent coordinates in OpenGL space that fill your window, or
/// velocity, or whatever other two f32 values you need.  The OpenGL window is (-1.0, -1.0) in
/// the bottom left to (1.0, 1.0) in the top right.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Vector2 {
    pub x : f32,
    pub y : f32,
}

impl Vector2 {
    /// New Vector2D at (0.0, 0.0)
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    /// Create a random Vector2D with x and y both in `[-dimension, dimension]`
    pub fn new_in_square<T: Rng>(dimension : f32, rng : &mut T) -> Self {
        Self {
            x: rng.gen_range(-dimension, dimension),
            y: rng.gen_range(-dimension, dimension),
        }
    }
    /// Calculate the distance between two Vector2's -- useful when they represent coordinates
    pub fn distance_between(&self, other : Self) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
    /// Calculate the angle between two Vector2's -- useful when they represent coordinates
    pub fn angle_between(&self, other : Self) -> f32 {
        (self.x - other.x).atan2(self.y - other.y)
    }
    /// Calculate the magnitude of the Vector2 -- useful when it represents a vector (such as
    /// velocity)
    pub fn magnitude(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    /// Create a new Vector2D, normalized to be of unit length (length 1).
    pub fn normalized(&self) -> Self {
        let magnitude = self.magnitude();
        Self {
            x: self.x / magnitude,
            y: self.y / magnitude,
        }
    }
    /// Create a new Vector2D that is clamped to a magnitude of `1.0`
    pub fn clamped_to_normal(&self) -> Self {
        if self.magnitude() > 1.0 {
            self.normalized()
        } else {
            self.clone()
        }
    }
    /// Create a new Vector2D clamped to `magnitude`
    pub fn clamped_to(&self, magnitude : f32) -> Self {
        if self.magnitude() > magnitude {
            let ratio = magnitude / self.magnitude();
            Self {
                x: self.x * ratio,
                y: self.y * ratio,
            }
        } else {
            self.clone()
        }
    }
}

/// Do docs for trait impls show up???
impl PartialOrd for Vector2 {
    fn partial_cmp(&self, other: &Vector2) -> Option<Ordering>{
        let magnitude = self.magnitude();
        let other_magnitude = other.magnitude();
        return if magnitude < other_magnitude {
            Some(Ordering::Less)
        } else if magnitude == other_magnitude {
            Some(Ordering::Equal)
        } else {
            Some(Ordering::Greater)
        };
    }
}

impl Add for Vector2 {
    type Output = Vector2;
    fn add(self, other : Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, other : f32) -> Vector2 {
        Vector2 {
            x: self.x * other,
            y: self.y * other,
        }
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
    MouseMoved { position : Vector2 },
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
    /// Version number of the server you are connecting to. Compare to rusty_sword_arena::version
    pub version : String,
    /// The ID of your player.
    pub your_player_id : u8,
    /// The maximum amount of players this server will allow
    pub max_players : u8,
    /// OpenGL units. Collision radius of players (size of a player)
    pub player_radius : f32,
    /// How quickly a player can get moving (magnitude in OpenGL units per second^2)
    pub acceleration : f32,
    /// Maximum velocity of a player (magnitude in OpenGL units per second)
    pub max_velocity : f32,
    /// How much drag there is on movement when the player is _not_ trying to move (how quick you
    /// stop)
    pub drag : f32,
    /// Move threshold. Magnitude of Vector2 below which a move_speed will be considered 0.
    pub move_threshold : f32,
    /// Milliseconds. How long the server will wait to respawn a player who dies.
    pub respawn_delay : u64,
    /// Milliseconds. How long the server will allow not receiving input before dropping a player.
    pub drop_delay : u64,
    /// Milliseconds. How long you have to wait before the server will let you perform another attack.
    pub attack_delay : u64,
    /// Map of player id to settings such as names and colors. Note that this includes your own name
    /// _which may not be what you requested originally!_.
    pub player_settings : HashMap<u8, PlayerSetting>,
}

impl GameSetting {
    pub fn new() -> Self {
        GameSetting {
            version : VERSION.to_string(),
            your_player_id : 0,
            max_players : 32,
            player_radius : 0.05,
            acceleration : 1.5,
            max_velocity : 0.25,
            drag : 5.0,
            move_threshold : 0.05,
            respawn_delay : 5000,
            drop_delay : 4000,
            attack_delay : 500,
            player_settings : HashMap::<u8, PlayerSetting>::new(),
        }
    }
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
        (self.acceleration as u32).hash(state);
        (self.drag as u32).hash(state);
        (self.respawn_delay as u32).hash(state);
        self.drop_delay.hash(state);
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
    /// Player has joined the game
    Join,
    /// Player has left the game
    Leave,
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
            damage : 17.0,
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
    pub pos : Vector2,
    /// The direction the player is facing, in radians
    pub direction: f32,
    /// Current velocity of the player
    pub velocity : Vector2,
    /// Current health of the player [0.0, 100.0]
    pub health : f32,
    starting_health : f32,
    /// Current regen rate of the player
    pub regen : f32,
    /// Current weapon of the player
    pub weapon : Weapon,
    /// Any player events that have occurred to the player this frame
    pub player_events : Vec<PlayerEvent>,
    /// How long until the player can attack again
    pub attack_timer : Timer,
    /// How long the server will wait to get input from you before disconnecting you
    pub drop_timer : Timer,
    /// How long until the player respawns.  If respawn_timer.ready == false, then the player is
    /// dead and you should seriously consider indicating that visually somehow, even if only by not
    /// displaying the player.
    pub respawn_timer : Timer,
    /// Are you dead?  Untangling health/respawn_timer dynamics is a pain, so we'll use this much
    /// more convenient boolean.
    pub dead : bool,
}

impl PlayerState {
    pub fn new(game_setting : &GameSetting) -> Self {
        // Manually pump the respawn timer so it's wound-down.
        let mut respawn_timer = Timer::from_millis(game_setting.respawn_delay);
        respawn_timer.set_millis_transient(1000); // spawn more quickly on initial connect
        Self {
            id : 0,
            pos : Vector2::new(),
            direction: 0.0,
            velocity : Vector2::new(),
            health : 100.0,
            starting_health : 100.0,
            regen : 0.0,
            weapon : Weapon::new(),
            player_events : Vec::<PlayerEvent>::new(),
            attack_timer : Timer::from_millis(game_setting.attack_delay),
            drop_timer: Timer::from_millis(game_setting.drop_delay),
            respawn_timer,
            dead : true,
        }
    }
    pub fn update(&mut self, delta : Duration) {
        self.attack_timer.update(delta);
        self.drop_timer.update(delta);
        self.respawn_timer.update(delta);
    }
    /// Used by the server to reset things that have been taken care of last frame.
    pub fn new_frame(&mut self) {
        self.player_events.clear();
    }
    /// Used by the server when a player needs to die
    pub fn die(&mut self, msg : &str) {
        println!("{}", msg);
        self.health = -1.0;
        self.respawn_timer.reset();
        self.player_events.push(PlayerEvent::Die);
        self.dead = true;
    }
    /// Used by the server when a player needs to spawn
    pub fn respawn(&mut self, pos : Vector2, msg : &str) {
        println!("{}", msg);
        self.pos = pos;
        self.health = self.starting_health;
        self.player_events.push(PlayerEvent::Spawn);
        self.dead = false;
    }
}

/// Once per frame, the server will broadcast a GameState to all clients.  IMPORTANT: If you don't
/// receive a GameState for 2 full seconds, the client MUST DROP its ServerConnection (or just
/// exit entirely).  The underlying networking that we're currently using hides network disconnects,
/// which can leave the networking in a funky state if one end drops.  So we need to rely on
/// detecting this heartbeat and shutting down the clients to ensure networking is clean when the
/// server restarts.  (In the future, lets switch to a protocol that can detect disconnects...)
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
    /// How much your player is attempting to move horizontally (x) and vertically (y) [-1.0, 1.0].
    /// Positive is right and up for x and y, respectively.
    pub move_amount : Vector2,
    /// What direction your player is facing. You can turn instantly, you lucky dog.
    pub direction: f32,
}

impl PlayerInput {
    pub fn new() -> Self {
        Self {
            id : 0,
            attack : false,
            move_amount : Vector2::new(),
            direction: 0.0,
        }
    }
    // Combine successive inputs into one input
    pub fn coalesce(&mut self, new : PlayerInput) {
        // Any attack sticks
        self.attack = self.attack || new.attack;
        // Anything else the new value wins
        self.move_amount = new.move_amount;
        self.direction = new.direction;
    }
}

