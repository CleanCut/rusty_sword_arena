//! 💡 **Did You Know?** You can generate your own, offline version of this document by running this
//! command in your own clone of the repository.
//!
//! ```bash
//! cargo doc --lib --no-deps --open
//! ```
//!
//! **Other Tutorial References**
//!
//! - [Git repository for Rusty Sword Arena](https://github.com/CleanCut/rusty_sword_arena) repository.
//! - [How To Learn Rust]() - a handy reference.
//!
//! # Rusty Sword Arena - Game Design
//!
//! Rusty Sword Arena is a networked, 2D, top-down, arena combat game.  A server implementation and
//! shared library is provided.  Your task is to implement a game client in Rust using the provided
//! shared library and the game design details below.
//!
//! Note that while significant effort has been put into making the server _robust_ and unlikely to
//! crash, there has consciously been very little effort towards security, or pretty much any proper
//! game engine architecture.  In other words, this is a great way to learn Rust and have some fun
//! at the same time, but this in no way pretends to be a substitute for a _real game engine™._ 😉
//!
//! ## Basic Gameplay
//!
//! The basic idea of the game is you are presented with the top-down view of a person (circle) in
//! an arena (your window) who can swing his weapon (a rusty sword) around a certain distance (a
//! larger [concentric] circle than the person himself).  You can face any direction you like, in
//! fact turning is instant as far as the server is concerned.  You can move in any direction (which
//! is _not_ instant, the server implements some rudimentary movement physics) and attempt to attack
//! other players with your sword.  You will be limited to 50% movement speed while attacking, so it
//! is to your benefit to try to time your attack attempts to when you are actually ready to attack.
//!
//! You start with some health. When you are hit, you lose health.  When the health runs out, you
//! die and the person who killed you gets a point.  After a respawn delay, you respawn with full
//! health.  If you attempt to run from the arena, you are likely to be eaten by a grue.  Being
//! eaten by a grue causes you to lose a point, but there is no point penalty when another player
//! kills you.
//!
//! ## Preparation
//!
//! First, you should follow the [instructions on the readme](https://github.com/cleancut/rusty_sword_arena)
//! to install some prerequisites.  Then come back here and keep going.
//!

//!
//! ## Creating Your Game Client
//!
//! - Pick a name for your client and create a Rust crate.  Hint: use `cargo`
//! - Add `rusty_sword_arena` as a dependency in your `Cargo.toml` file.
//! - Obtain the player's desired `name` and the `host` to connect to.
//!   - One easy way is to get the name and host from the command-line.
//!   - See [args](https://doc.rust-lang.org/std/env/fn.args.html) for the code part.
//! - Create a [ServerConnection](struct.ServerConnection.html) using the `host`
//! - Use the [ServerConnection](struct.ServerConnection.html) to join the game
//!   - You should probably hold onto that ID so you know which player you are.
//! - Use the [ServerConnection](struct.ServerConnection.html) to get a
//!   [GameSetting](game/struct.GameSetting.html).  If `version` in game setting doesn't match
//!   [VERSION](constant.VERSION.html) in the rusty_sword_arena module you are using, you may want
//!   to fix your `Cargo.toml`
//! ...
//!

#![doc(html_favicon_url = "https://agileperception.com/static/img/favicon.ico")]
#![doc(html_logo_url = "https://agileperception.com/static/img/APSwirl200.png")]
extern crate bincode;
#[macro_use]
extern crate glium;
extern crate rand;
#[macro_use]
extern crate serde_derive;
extern crate zmq;

/// Everything in the game module is shared by the server _and_ the client
pub mod game;
/// The graphics module that will be used by your client
pub mod gfx;
/// The networking module that will be used by your client
pub mod net;
/// A timer module for general use
pub mod timer;

/// The current version number. Your client should check this against the version the server sends
/// in [GameSettings]()
pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
