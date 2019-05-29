use zmq;
use super::game::{GameControlMsg, GameSetting, GameState, PlayerInput};
use bincode::{deserialize, serialize};

#[doc(hidden)]
pub const PLAYER_INPUT_PORT: i32 = 8001;
#[doc(hidden)]
pub const GAME_STATE_PORT: i32 = 8002;
#[doc(hidden)]
pub const GAME_CONTROL_PORT: i32 = 8003;

/// Represents a client's connection _to_ a server, and methods to abstract away all the actual
/// network communication. Hooray for encapsulation!
pub struct ServerConnection {
    _context: zmq::Context,
    game_control_socket: zmq::Socket,
    game_state_socket: zmq::Socket,
    player_input_socket: zmq::Socket,
}

impl ServerConnection {
    /// Create a new connection to a server.  `host` is the IP address or domain name of the server.
    pub fn new(host: &str) -> Self {
        let context = zmq::Context::new();

        let game_control_socket = context.socket(zmq::REQ).unwrap();
        game_control_socket
            .connect(&format!("tcp://{}:{}", host, GAME_CONTROL_PORT))
            .unwrap();

        let game_state_socket = context.socket(zmq::SUB).unwrap();
        game_state_socket.set_rcvtimeo(0).unwrap();
        game_state_socket
            .connect(&format!("tcp://{}:{}", host, GAME_STATE_PORT))
            .unwrap();
        game_state_socket.set_subscribe(&[]).unwrap();

        let player_input_socket = context.socket(zmq::PUSH).unwrap();
        player_input_socket
            .connect(&format!("tcp://{}:{}", host, PLAYER_INPUT_PORT))
            .unwrap();

        Self {
            _context: context,
            game_control_socket,
            game_state_socket,
            player_input_socket,
        }
    }

    /// Join a game.  If successful, you'll get a non-zero player id back.  If not successful,
    /// there was probably a name collision, so change your name and try again.  If changing the
    /// name still doesn't work, then the server is probably full.
    pub fn join(&mut self, name: &str) -> u8 {
        let msg = GameControlMsg::Join {
            name: name.to_string(),
        };
        self.game_control_socket
            .send(&serialize(&msg).unwrap(), 0)
            .unwrap();
        let bytes = self.game_control_socket.recv_bytes(0).unwrap();
        let new_id: u8 = deserialize(&bytes[..]).unwrap();
        new_id
    }

    /// Get the current GameSetting.  The most important thing about the GameSetting is checking
    /// that you are connecting to a version of the server you expect.
    pub fn get_game_setting(&mut self) -> GameSetting {
        let msg = GameControlMsg::Fetch;
        self.game_control_socket
            .send(&serialize(&msg).unwrap(), 0)
            .unwrap();
        let bytes = self.game_control_socket.recv_bytes(0).unwrap();
        let game_setting: GameSetting = deserialize(&bytes[..]).unwrap();
        game_setting
    }

    /// Leave the game.
    pub fn leave(&mut self, id: u8) -> bool {
        let msg = GameControlMsg::Leave { id };
        self.game_control_socket.set_rcvtimeo(1500).unwrap();
        self.game_control_socket
            .send(&serialize(&msg).unwrap(), 0)
            .unwrap();
        if let Ok(bytes) = self.game_control_socket.recv_bytes(0) {
            let succeeded: bool = deserialize(&bytes[..]).unwrap();
            return succeeded;
        }
        return false;
    }

    /// Gets all available unprocessed game states.  You should call this often enough that you
    /// usually don't receive more than one.
    pub fn poll_game_states(&mut self) -> Vec<GameState> {
        let mut game_states = Vec::<GameState>::new();
        while let Ok(bytes) = self.game_state_socket.recv_bytes(0) {
            game_states.push(deserialize(&bytes[..]).unwrap());
        }
        game_states
    }

    /// Send player input to the server. The server processes input as it comes in, but that doesn't
    /// mean you should send 10,000 input packets/second.  Keep track of the input and only send
    /// new input about every 15ms.
    pub fn send_player_input(&mut self, player_input: PlayerInput) {
        self.player_input_socket
            .send(&serialize(&player_input).unwrap(), 0)
            .unwrap();
    }
}
