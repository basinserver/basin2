pub mod game;
pub use game::*;
pub mod handshake;
pub use handshake::*;
pub mod login;
pub use login::*;
pub mod status;
pub use status::*;

use bytes::BytesMut;
use crate::packet::PacketContainer;

pub enum Packet {
    PacketGame(PacketGame),
    PacketHandshake(PacketHandshake),
    PacketLogin(PacketLogin),
    PacketStatus(PacketStatus),
}

impl Packet {
    pub fn encode(self, buf: &mut BytesMut) {
        use Packet::*;
        use game::PacketGame::*;
        use handshake::PacketHandshake::*;
        use login::PacketLogin::*;
        use status::PacketStatus::*;
        
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
