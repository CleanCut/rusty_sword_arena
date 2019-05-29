use super::timer::Timer;
use super::VERSION;

use rand::prelude::Rng;
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::Add;
use std::ops::Mul;
use std::time::Duration;

/// 2D Vector (x, y) that can represent coordinates in OpenGL space that fill your window, or
/// velocity, or whatever other two f32 values you need.  The OpenGL window is (-1.0, -1.0) in
/// the bottom left to (1.0, 1.0) in the top right.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Vector2 {
    pub x: f32,
    pub y: f32,
}

impl Vector2 {
    /// New Vector2D at (0.0, 0.0)
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0 }
    }
    /// Create a random Vector2D with x and y both in `[-dimension, dimension]`
    pub fn new_in_square<T: Rng>(dimension: f32, rng: &mut T) -> Self {
        Self {
            x: rng.gen_range(-dimension, dimension),
            y: rng.gen_range(-dimension, dimension),
        }
    }
    /// Calculate the distance between two Vector2's -- useful when they represent coordinates
    pub fn distance_between(&self, other: Self) -> f32 {
        ((self.x - other.x).powi(2) + (self.y - other.y).powi(2)).sqrt()
    }
    /// Calculate the angle between two Vector2's -- useful when they represent coordinates
    pub fn angle_between(&self, other: Self) -> f32 {
        (other.y - self.y).atan2(other.x - self.x)
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
    pub fn clamped_to(&self, magnitude: f32) -> Self {
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
    fn partial_cmp(&self, other: &Vector2) -> Option<Ordering> {
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
    fn add(self, other: Vector2) -> Vector2 {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, other: f32) -> Vector2 {
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

/// Abstracted button values you may receive (arrow keys and WASD keys combined into directions, for
/// example)
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ButtonValue {
    /// An abstracted button that combines: Arrow Up, W, Comma (Dvorak)
    Up,
    /// An abstracted button that combines: Arrow Down, S, O (Dvorak)
    Down,
    /// An abstracted button that combines: Arrow Left, A
    Left,
    /// An abstracted button that combines: Arrow Right, D, E (Dvorak)
    Right,
    /// An abstracted button that combines: Left Mouse Button, Space Bar, Backspace (Kinesis
    /// Advantage Keyboard)
    Attack,
    /// An abstracted button that combines: Escape
    Quit,
}

/// Whether a button was pressed or released
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum ButtonState {
    /// A button was just pressed
    Pressed,
    /// A button was just released
    Released,
}

/// `InputEvent` represents input based on the window that is being displayed, such as someone
/// closing the window, the mouse moving around, or buttons being pushed.
#[derive(Copy, Clone, Debug, Deserialize, Serialize, PartialEq)]
pub enum InputEvent {
    /// The window was closed somehow, so we better quit the application...unless we're going to
    /// pop up another window.
    WindowClosed,
    /// Indicates the current position the mouse has moved to.  The mouse is now at this location in
    /// OpenGL coordinates.  Note that on some operating systems this event will fire even if the
    /// cursor is outside the bounds of the window.
    MouseMoved { position: Vector2 },
    /// Indicates that a button with value `ButtonValue` has been either pressed or released
    /// (`ButtonState`).  Note that both mouse buttons and keyboard buttons are abstracted and
    /// collected together into a few logical game buttons.
    /// See [ButtonValue](game/enum.ButtonValue.html)
    Button {
        button_value: ButtonValue,
        button_state: ButtonState,
    },
}

/// Various game control actions. Used by the networking module and the server.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum GameControlMsg {
    Join { name: String },
    Leave { id: u8 },
    Fetch,
}

/// A color with 32-bit float parts from `[0.0, 1.0]` suitable for OpenGL.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Color {
    /// Red
    pub r: f32,
    /// Green
    pub g: f32,
    /// Blue
    pub b: f32,
}

impl Color {
    /// Slightly simpler way to create a color
    pub fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }
}

impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (self.r as u32).hash(state);
        (self.g as u32).hash(state);
        (self.b as u32).hash(state);
    }
}

impl Eq for Color {}

/// The game setting.  Mostly useful if you want to try to write client-side movement prediction,
/// AI, etc.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GameSetting {
    /// Version number of the server you are connecting to. Compare to rusty_sword_arena::version
    pub version: String,
    /// The maximum amount of players this server will allow
    pub max_players: u8,
    /// How quickly a player can get moving (magnitude in OpenGL units per second^2)
    pub acceleration: f32,
    /// Maximum velocity of a player (magnitude in OpenGL units per second)
    pub max_velocity: f32,
    /// How much drag there is on movement when the player is _not_ trying to move (how quick you
    /// stop)
    pub drag: f32,
    /// Move threshold. Magnitude of Vector2 below which a move_speed will be considered 0.
    pub move_threshold: f32,
    /// Milliseconds. How long the server will wait to respawn a player who dies.
    pub respawn_delay: u64,
    /// Milliseconds. How long the server will allow not receiving input before dropping a player.
    pub drop_delay: u64,
}

impl GameSetting {
    /// Create the default game settings
    pub fn new() -> Self {
        GameSetting {
            version: VERSION.to_string(),
            max_players: 64,
            acceleration: 1.5,
            max_velocity: 0.25,
            drag: 5.0,
            move_threshold: 0.05,
            respawn_delay: 5000,
            drop_delay: 4000,
        }
    }
    /// Looking forward to the day when game settings can be changed mid-game.
    pub fn get_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.hash(&mut hasher);
        hasher.finish()
    }
}

impl Hash for GameSetting {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.version.hash(state);
        self.max_players.hash(state);
        (self.acceleration as u32).hash(state);
        (self.drag as u32).hash(state);
        (self.move_threshold as u32).hash(state);
        self.respawn_delay.hash(state);
        self.drop_delay.hash(state);
    }
}

/// A single player's score
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Score {
    name: String,
    points: i32,
}

impl Score {
    /// Create a new score
    fn new(name: &str, points: i32) -> Self {
        Self {
            name: name.to_string(),
            points: points,
        }
    }
}

impl fmt::Display for Score {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:<4.0} {}", self.points, self.name)
    }
}

impl Eq for Score {}

impl Ord for Score {
    fn cmp(&self, other: &Score) -> Ordering {
        if self.points < other.points {
            Ordering::Less
        } else if self.points == other.points {
            if self.name < other.name {
                Ordering::Greater
            } else if self.name == other.name {
                Ordering::Equal
            } else {
                Ordering::Less
            }
        } else {
            Ordering::Greater
        }
    }
}

impl PartialOrd for Score {
    fn partial_cmp(&self, other: &Score) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// High Scores!  High scores are reset every time the server restarts, but other than that they
/// are persistent across joins/leaves.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct HighScores {
    pub scores: Vec<Score>,
}

impl HighScores {
    /// Create a new, blank set of high scores
    pub fn new() -> Self {
        Self {
            scores: Vec::<Score>::new(),
        }
    }
    /// Bump a player's score up by one. If the player is not present, he will be added.
    pub fn score(&mut self, name: &str) {
        self.add_player(name);
        if let Some(score) = self.scores.iter_mut().find(|x| x.name == name) {
            score.points += 1;
        }
        self.sort();
    }
    /// Decrement a player's score by one. If the player is not present, he will be added.
    pub fn penalize(&mut self, name: &str) {
        self.add_player(name);
        if let Some(score) = self.scores.iter_mut().find(|x| x.name == name) {
            score.points -= 1;
        }
        self.sort();
    }
    /// Return a clone with only the top-10 scoring players. This is what the server sends the
    /// client.
    pub fn top10(&self) -> Self {
        let mut top10 = self.clone();
        while top10.scores.len() > 10 {
            top10.scores.pop();
        }
        top10
    }
    /// Add a new player with a zero score. Used internally by both `score()` and `penalize()`
    pub fn add_player(&mut self, name: &str) {
        // Abort if we've already seen this player.
        if self.scores.iter().any(|x| x.name == name) {
            return;
        }
        self.scores.push(Score::new(name, 0));
        self.sort();
    }
    // Sort the internal score vector in the direction we want.
    fn sort(&mut self) {
        self.scores.sort_by(|a, b| b.cmp(a));
    }
}

impl fmt::Display for HighScores {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let _ = write!(f, "-----------\nHigh Scores\n-----------");
        for score in self.scores.iter() {
            let _ = write!(f, "\n{}", score);
        }
        Ok(())
    }
}

/// A player event that has happened to your player this frame!  Note that it's possible to receive
/// a whole bunch of events in the same frame.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub enum PlayerEvent {
    /// Player has attacked and hit player id.
    AttackHit { id: u8 },
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

/// A weapon a player may hold.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Weapon {
    /// Something like "Rusty Sword", "Shiny Sword", "Rusty Spear", etc.
    pub description: String,
    /// How much damage the weapon can cause
    pub damage: f32,
    /// How long until the player can attack again
    pub attack_timer: Timer,
    /// How far attacks reach from your player, in OpenGL units.
    pub radius: f32,
}

impl Weapon {
    pub fn new() -> Self {
        Self {
            description: "Rusty Sword".to_string(),
            damage: 17.0,
            radius: 0.1,
            attack_timer: Timer::from_millis(500),
        }
    }
}

/// The state of a player on the server. The server broadcasts these to all clients every frame as
/// part of a FrameState.  Note that you can receive `PlayerState`s before you have gotten a
/// corresponding GameSetting telling you their name and color!
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PlayerState {
    /// The ID of the player
    pub id: u8,
    /// The name of the player
    pub name: String,
    /// The color of the player
    pub color: Color,
    /// The position of the player in OpenGL units.
    pub pos: Vector2,
    /// The direction the player is facing, in radians
    pub direction: f32,
    /// Your player occupies a circle of this radius, in OpenGL units.
    pub radius: f32,
    /// Current velocity of the player
    pub velocity: Vector2,
    /// Current health of the player [0.0, 100.0]
    pub health: f32,
    // Private! No docs for you.  We use this when we respawn.
    starting_health: f32,
    /// Current weapon of the player
    pub weapon: Weapon,
    /// Any player events that have occurred to the player this frame
    pub player_events: Vec<PlayerEvent>,
    /// How long the server will wait to get input from you before disconnecting you
    pub drop_timer: Timer,
    /// How long until the player respawns.  If respawn_timer.ready == false, then the player is
    /// dead and you should seriously consider indicating that visually somehow, even if only by not
    /// displaying the player.
    pub respawn_timer: Timer,
    /// Are you dead?  Untangling health/respawn_timer dynamics is a pain, so we'll use this much
    /// more convenient boolean.
    pub dead: bool,
}

/// Represents the state of the player on the server for the current frame.  Always delivered by the
/// server to the client inside a [GameState](game/struct.GameState.html).  The server always
/// creates `PlayeState`s, updates them, and sends them to the client each frame. The client is free
/// to modify its local copy (for example, to remove [PlayerEvent](game/enum.PlayerEvent.html)s it
/// has processed) -- a new set of `PlayerStates` will be delivered the next frame. Clients
/// typically look at the fields and player events, and don't use any of the methods.
impl PlayerState {
    /// The client should never create a `PlayerState` -- the server will do that.
    pub fn new(
        game_setting: &GameSetting,
        id: u8,
        name: String,
        color: Color,
        pos: Vector2,
        radius: f32,
    ) -> Self {
        let mut respawn_timer = Timer::from_millis(game_setting.respawn_delay);
        respawn_timer.set_millis_transient(1000); // spawn more quickly on initial connect
        Self {
            id,
            name,
            color,
            pos,
            direction: 0.0,
            radius,
            velocity: Vector2::new(),
            health: 100.0,
            starting_health: 100.0,
            weapon: Weapon::new(),
            player_events: Vec::<PlayerEvent>::new(),
            drop_timer: Timer::from_millis(game_setting.drop_delay),
            respawn_timer,
            dead: true,
        }
    }
    /// Clients never need to call update. The server calls it each time it processes some amount of
    /// time.
    pub fn update(&mut self, delta: Duration) {
        self.weapon.attack_timer.update(delta);
        self.drop_timer.update(delta);
        self.respawn_timer.update(delta);
    }
    /// Used by the server to reset things that have been taken care of last frame.
    pub fn new_frame(&mut self) {
        self.player_events.clear();
    }
    /// Used by the server when a player needs to die
    pub fn die(&mut self, msg: &str) {
        println!("{}", msg);
        self.health = -1.0;
        self.respawn_timer.reset();
        self.player_events.push(PlayerEvent::Die);
        self.dead = true;
    }
    /// Used by the server when a player needs to spawn
    pub fn respawn(&mut self, pos: Vector2, msg: &str) {
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
    pub frame_number: u64,
    /// The actual time the server measured since the previous frame.
    pub delta: Duration,
    /// The hash of the current game setting. Your client should store this somewhere. If it changes
    /// then something has changed (most likely a player has joined or disconnected), so you should
    /// send a GameControlMsg::Fetch to get the new GameSetting from the server and update your
    /// client state.
    pub game_setting_hash: u64,
    /// All of the current player's states, including your own! **NOTE:** The only reliable method
    /// of knowing that a player is present in the game or not is whether or not a state is in
    /// player_states.  If there isn't a state, then the player has left or has been kicked/dropped.
    /// If there is a state, then the player is in the game (and might have joined since the last
    /// GameState was sent).
    pub player_states: HashMap<u8, PlayerState>,
    /// High scores. The server will only send the top 10.
    pub high_scores: HighScores,
}
/// Clients should send `PlayerInput`s to the server often.  The quicker the server gets inputs, the
/// more accurate the simulation will be.  But of course, you also shouldn't overload the server
/// with too much traffic, because that's bad too.  Good rule of thumb: Coalesce 15 milliseconds
/// worth of input together, and send that.  That's just faster than frames are sent by the
/// server (60fps = 16.7ms).  The server should be able to handle ~67 pkts/sec per client.  I hope.
///
/// `PlayerInput` is used to collect input into and send it to the server.  You
/// can use the [angle_between](game/struct.Vector2.html#method.angle_between) method of a
/// Vector2 to find the direction for the input based off of the position in your own player's
/// [PlayerState](game/struct.PlayerState.html) and the current position of the mouse,
/// which you get from one of the [input events](gfx/struct.Window.html#method.events)
///
/// Note that `attack` is an indicator of whether the player is currently (still) attempting to
/// attack. The server will only attack once every once-in-awhile when the weapon's attack timer
/// is ready, so you probably want to keep attack on while an attack button is held down, and
/// then switch attack off when the attack button is released.
///
/// It's pretty safe to just overwrite move_amount and direction with the latest input you've got.
/// Direction handling is instantaneous, and the move_amount is treated like a force (you move with
/// a bit of inertia).
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct PlayerInput {
    /// The ID of your player
    pub id: u8,
    /// Whether you are attempting to attack (actual attack will occur if the server-side attack
    /// timer has reached 0)
    pub attack: bool,
    /// How much your player is attempting to move horizontally (x) and vertically (y) [-1.0, 1.0].
    /// Positive is right and up for x and y, respectively.  You can derive movement amounts from
    /// [Button](game/enum.InputEvent.html#variant.Button) variants of the
    /// [InputEvent](game/enum.InputEvent.html)s you get from the
    /// [Window](gfx/struct.Window.html#method.poll_input_events).
    pub move_amount: Vector2,
    /// What direction your player is facing. You can turn instantly, you lucky dog.
    pub direction: f32,
}

impl PlayerInput {
    /// Create a new PlayerInput
    pub fn new() -> Self {
        Self {
            id: 0,
            attack: false,
            move_amount: Vector2::new(),
            direction: 0.0,
        }
    }
    /// Used by the server. Unlikely to be used by the client.
    pub fn coalesce(&mut self, new: PlayerInput) {
        // Any attack sticks
        self.attack = self.attack || new.attack;
        // Anything else the new value wins
        self.move_amount = new.move_amount;
        self.direction = new.direction;
    }

}
