use crate::game::{GameControlMsg, GameSetting, GameState, PlayerInput};

use bincode::{deserialize, serialize};
use zmq;

#[doc(hidden)]
pub const PLAYER_INPUT_PORT: i32 = 8001;
#[doc(hidden)]
pub const GAME_STATE_PORT: i32 = 8002;
#[doc(hidden)]
pub const GAME_CONTROL_PORT: i32 = 8003;

/// This is your client's network connection to the server. The methods abstract
/// away all the actual object serialization and network communication. Hooray
/// for encapsulation!
pub struct ConnectionToServer {
    _context: zmq::Context,
    game_control_socket: zmq::Socket,
    game_state_socket: zmq::Socket,
    player_input_socket: zmq::Socket,
}

impl ConnectionToServer {
    /// Create a new connection to a server.  `host` is the IP address or domain
    /// name of the server.  If you run the server on the same machine, you
    /// should pass `localhost` or `127.0.0.1` as the host.
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

    /// Join a game.  If successful, you'll get a non-zero player id back. Save
    /// the player id, because it is how you will be able to tell which of the
    /// players the server tells you about is YOU! If unsuccessful then there
    /// was probably a name collision, so change your name and try again.  If
    /// changing the name still doesn't work, then the server is probably full.
    /// TODO: Return a Result with a nice Error.
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

    /// Get the current GameSetting.  You should look at the version number and
    /// make sure that you are connecting to a version of the server you expect.
    pub fn get_game_setting(&mut self) -> GameSetting {
        let msg = GameControlMsg::Fetch;
        self.game_control_socket
            .send(&serialize(&msg).unwrap(), 0)
            .unwrap();
        let bytes = self.game_control_socket.recv_bytes(0).unwrap();
        let game_setting: GameSetting = deserialize(&bytes[..]).unwrap();
        game_setting
    }

    /// Cause the selected player id to leave the game.  You should pass in your
    /// own player id, obviously.  Passing in someone else's player id would be
    /// really mean.
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

    /// Gets all available unprocessed game states.  Game states arrive in
    /// order.  You should call this every time around your game loop.
    pub fn poll_game_states(&mut self) -> Vec<GameState> {
        let mut game_states = Vec::<GameState>::new();
        while let Ok(bytes) = self.game_state_socket.recv_bytes(0) {
            game_states.push(deserialize(&bytes[..]).unwrap());
        }
        game_states
    }

    /// Send player input to the server. The server processes input as it comes
    /// in, but that doesn't mean you should send 10,000 input packets/second.
    /// Keep track of the input and only send new input about every 15ms.
    pub fn send_player_input(&mut self, player_input: PlayerInput) {
        self.player_input_socket
            .send(&serialize(&player_input).unwrap(), 0)
            .unwrap();
    }
}
