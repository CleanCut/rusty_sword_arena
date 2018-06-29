extern crate zmq;
use super::{GameControlMsg, GameSetting, GameState, PlayerInput};
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
    _context : zmq::Context,
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
        game_state_socket.set_rcvtimeo(0).unwrap();
        game_state_socket.connect(&format!("tcp://{}:{}", server_host, GAME_STATE_PORT)).unwrap();
        game_state_socket.set_subscribe(&[]).unwrap();

        let player_input_socket = context.socket(zmq::PUSH).unwrap();
        player_input_socket.connect(&format!("tcp://{}:{}", server_host, PLAYER_INPUT_PORT)).unwrap();

        Self {
            _context : context,
            game_control_socket,
            game_state_socket,
            player_input_socket
        }
    }

    /// Send a control message to join, leave, or sync a game. Get back GameSetting.
    pub fn send_game_control(&mut self, msg : GameControlMsg) -> Result<GameSetting, Box<Error>> {
        self.game_control_socket.send(&serialize(&msg)?, 0)?;
        let bytes = self.game_control_socket.recv_bytes(0)?;
        let game_settings : GameSetting = deserialize(&bytes[..])?;
        Ok(game_settings)
    }

    /// Gets all available unprocessed game states.  You should call this often enough that you
    /// usually don't receive more than one.
    pub fn recv_game_states(&mut self) -> Vec<GameState> {
        let mut game_states = Vec::<GameState>::new();
        while let Ok(bytes) = self.game_state_socket.recv_bytes(0) {
            game_states.push(deserialize(&bytes[..]).unwrap());
        }
        game_states
    }

    /// Send player input to the server. The server processes input as it comes in, so you should
    /// send it ASAP to reduce lag and improve the simulation.
    pub fn send_player_input(&mut self, player_input : PlayerInput) {
        self.player_input_socket.send(&serialize(&player_input).unwrap(), 0).unwrap();
    }
}
