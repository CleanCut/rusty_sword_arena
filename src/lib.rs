/// The endpoint for clients to request (zmq::REQ) to the server's reply (zmq::REP) socket.
/// Clients will send their updates to the server this way.
pub const client_port : i32 = 8001;

/// The endpoint for clients to subscribe (zmq::SUB) to the server's publish (zmq::PUB) socket and
/// receive game state updates.
pub const game_port : i32 = 8002;


pub mod prelude {
    pub use super::{client_port, game_port};
}