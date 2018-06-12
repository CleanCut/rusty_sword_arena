# Rusty Sword Arena - Game Design

Rusty Sword Arena is a networked, 2D, top-down, arena combat game.  A server implementation and shared library is 
provided.  Your task is to implement a game client in Rust using the provided shared library and the game design details 
below.

Note that while significant effort has been put into making the server _robust_ and unlikely to crash, there has
consciously been very little effort towards security, or pretty much any proper game engine architecture.  In other 
words, this is a great way to learn Rust and have some fun at the same time, but this in no way pretends to be a 
substitute for a real™ game engine. 😉


`rsa::` is used for an abbreviation of `rusty_sword_arena::`. You can actually make that alias if you like:

```rust
extern crate rusty_sword_arena;
use rusty_sword_arena as rsa;
```

## Join/Sync/Leave a Game

To join, sync, or leave a game, the game client should establish a request/reply channel to to the game settings port 
and send a PlayerSync message, upon which you will receive a GameSettings reply.


#### PlayerSync

| var | type | unit | description |
| --- | ---- | ---- | ----------- |
| name_request | `String` | | Desired name of your character. Note that the server may arbitrarily assign a different name if it so chooses. |
| id | `u8` | | Specify `0` when joining. The id of your player if syncing or leaving. |
| action | `rsa::PlayerSyncAction` | | Whether you want to `Join`, `Sync`, or `Leave`. 


#### GameSettings


| var | type | unit | description |
| --- | ---- | ---- | ----------- |
| your_player_id | `u8` | | The id of your player if > 0. |
| player_radius | `f64` | OpenGL units | Collision radius of players (size of a player) |
| move_dampening | `f64` | Percentage `[0.0, 1.0]` | How much the server will dampen your player's movement if moving exactly backwards.  Dampening effect is zero when moving exactly forwards, and linearly scales in movement directions between straight forward and straight backward. |
| frame_delay | `f64` | Seconds | Server will never _send_ frame updates more frequently than this. When and how far apart they arrive is entirely up to the network. |
| respawn_delay | `f64` | Seconds | How long the server will wait to respawn a player who dies. |
| player_names | `HashMap<u8, String>` | | Map of player id to names, including your own name _which may have been arbitrarily changed_. |
| player_colors | `HashMap<u8, Color>` | | Map of player id to player colors, including your own assigned color. |
 

 