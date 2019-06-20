# Rusty Sword Arena - Game Overview

Rusty Sword Arena is a networked, 2D, top-down, arena combat game.  A server implementation and shared library is 
provided.  Your task is to implement a game client in Rust using the provided shared library and the game design details 
below.

Note that while significant effort has been put into making the server _robust_ and unlikely to crash, there has
consciously been very little effort towards security, or pretty much any proper game engine architecture.  In other 
words, this is a great way to learn Rust and have some fun at the same time, but this in no way pretends to be a 
substitute for a real™ game engine. 😉


### The Library

Most of the heavy lifting (networking, graphics, event system) is already 
implemented for you in the `rusty_sword_arena` library.  This is the same
library used by the server.  In fact, the server binary is implemented within
the library itself.

## The Server

The server is provided.  To run it, clone the `rusty_sword_arena` repository
and run `cargo run --bin server`

## The Client

We will implement the client together in the tutorial.  Feel free to stray from
the presentation and implement things your own way if you are feeling 
adventurous!

Basic Steps:

- Choose a name for your client and create the skeleton with `cargo new somename`
- Add `rusty_sword_arena = "1.1"`

## Controls