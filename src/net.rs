extern crate zmq;
use super::{GameControlMsg, GameSettings};
use std::string::ToString;
use bincode::{serialize, deserialize};
use std::boxed::Box;
use std::error::Error;

/// The server port for clients to push (zmq::PUSH) to the server's pull (zmq::PULL) socket.
/// Clients will continuously stream their player input updates to the server this way.
pub const PLAYER_INPUT_PORT: i32 = 8001;

/// The server port for clients to subscribe (zmq::SUB) to the server's publish (zmq::PUB) socket
/// and receive a continuous stream of game state updates.
pub const GAME_STATE_PORT: i32 = 8002;

/// The server port for clients to request (zmq::REQ) to the server's reply (zmq::REP) socket
/// to do occasional game syincing operations like joining, finding out about new players (syncing),
/// and leaving.
pub const GAME_CONTROL_PORT: i32 = 8003;


/// Represents a client's connection *to* a server, and methods to abstract away all the actual
/// network communication. Weee!
pub struct ServerConnection {
    context : zmq::Context,
    game_control_socket : zmq::Socket,
    game_state_socket : zmq::Socket,
    player_input_socket : zmq::Socket,
}

impl ServerConnection {
    /// Create a new connection to a server.  server_host is the IP address or fqdn of the server.
    pub fn new<T: ToString>(server_host : T) -> Self {
        let server_host = server_host.to_string();

        let context = zmq::Context::new();

        let game_control_socket = context.socket(zmq::REQ).unwrap();
        game_control_socket.connect(&format!("tcp://{}:{}", server_host, GAME_CONTROL_PORT)).unwrap();

        let game_state_socket = context.socket(zmq::SUB).unwrap();
        game_state_socket.connect(&format!("tcp://{}:{}", server_host, GAME_STATE_PORT)).unwrap();
        game_state_socket.set_subscribe(&[]).unwrap();

        let player_input_socket = context.socket(zmq::PUSH).unwrap();
        player_input_socket.connect(&format!("tcp://{}:{}", server_host, PLAYER_INPUT_PORT)).unwrap();

        Self {
            context,
            game_control_socket,
            game_state_socket,
            player_input_socket
        }
    }

    /// Send a control message to join, leave, or sync a game. Get back GameSettings.
    pub fn game_control(self: &mut Self, msg : GameControlMsg) -> Result<GameSettings, Box<Error>> {
        self.game_control_socket.send(&serialize(&msg).unwrap(), 0)?;
        let bytes = self.game_control_socket.recv_bytes(0)?;
        let game_settings : GameSettings = deserialize(&bytes[..])?;
        Ok(game_settings)
    }
}
