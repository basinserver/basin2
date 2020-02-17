pub mod game;
pub use game::*;
pub mod handshake;
pub use handshake::*;
pub mod login;
pub use login::*;
pub mod status;
pub use status::*;

use crate::packet::PacketContainer;
use bytes::BytesMut;

hierarchy! {
enum Packet {
    Game,
    Handshake,
    Login,
    Status,
}
}

impl Packet {
    pub fn encode(self, buf: &mut BytesMut) {
        use Packet::*;

        match self {
            Game(game::Game::GameClientbound(packet)) => packet.encode(buf),
            Game(game::Game::GameServerbound(packet)) => packet.encode(buf),
            Handshake(handshake::Handshake::HandshakeClientbound(packet)) => packet.encode(buf),
            Handshake(handshake::Handshake::HandshakeServerbound(packet)) => packet.encode(buf),
            Login(login::Login::LoginClientbound(packet)) => packet.encode(buf),
            Login(login::Login::LoginServerbound(packet)) => packet.encode(buf),
            Status(status::Status::StatusClientbound(packet)) => packet.encode(buf),
            Status(status::Status::StatusServerbound(packet)) => packet.encode(buf),
        }
    }
}
