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

#[derive(Clone, Debug, PartialEq)]
pub enum Packet {
    PacketGame(PacketGame),
    PacketHandshake(PacketHandshake),
    PacketLogin(PacketLogin),
    PacketStatus(PacketStatus),
}

impl Packet {
    pub fn encode(self, buf: &mut BytesMut) {
        use game::PacketGame::*;
        use handshake::PacketHandshake::*;
        use login::PacketLogin::*;
        use status::PacketStatus::*;
        use Packet::*;

        match self {
            PacketGame(PacketGameClientbound(packet)) => packet.encode(buf),
            PacketGame(PacketGameServerbound(packet)) => packet.encode(buf),
            PacketHandshake(PacketHandshakeClientbound(packet)) => packet.encode(buf),
            PacketHandshake(PacketHandshakeServerbound(packet)) => packet.encode(buf),
            PacketLogin(PacketLoginClientbound(packet)) => packet.encode(buf),
            PacketLogin(PacketLoginServerbound(packet)) => packet.encode(buf),
            PacketStatus(PacketStatusClientbound(packet)) => packet.encode(buf),
            PacketStatus(PacketStatusServerbound(packet)) => packet.encode(buf),
        }
    }
}
