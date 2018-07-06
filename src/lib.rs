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
//! The basic idea of the game is you are presented with the top-down view of a person (circle) in
//! an arena (your window) who can swing his weapon (probably a rusty sword) around a certain
//! distance (a larger [concentric] circle than the person himself).  You can face any direction you
//! like, in fact turning is instant as far as the server is concerned.  You can move in any
//! direction (which is _not_ instant) and attempt to attack other players with your sword which may
//! hit or miss, or you may try to parry, which if successful greatly increases the timeout for the
//! attacking player's next attack or parry.
//!
//! You start with some health. When you are hit, you lose health.  When the health runs out, you
//! die.  After a respawn delay, you! respawn with full health.  If you attempt to run from the
//! arena, you are likely to be eaten by a grue.
//!
//! ## Preparation
//!
//! First, you should follow the [instructions on the readme](https://github.com/cleancut/rusty_sword_arena)
//! to install some prerequisites.  Then come back here and keep going.
//!
//! ## Basics
//!
//! - You need to pick a name for your client and create a Rust crate.  Hint: use `cargo`
//! - You need to list `rusty_sword_arena` as a dependency in your `Cargo.toml` file.
//!
//! ## Game Client Responsibilities
//!
//! - Obtain the player's desired `name` and the `host` to connect to.
//! - Create a [ServerConnection](struct.ServerConnection.html) using the `host`
//! - Using the server connection, send a [GameControlMsg](game/enum.GameControlMsg.html) to join
//!   the game and save the [GameSetting](game/struct.GameSetting.html) you get back.
//! - Determine your player's id and save it
//! - ...
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
