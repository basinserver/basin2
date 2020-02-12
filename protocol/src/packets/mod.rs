pub mod game;
pub use game::*;
pub mod handshake;
pub use handshake::*;
pub mod login;
pub use login::*;
pub mod status;
pub use status::*;

pub enum Packet {
    PacketGame(PacketGame),
    PacketHandshake(PacketHandshake),
    PacketLogin(PacketLogin),
    PacketStatus(PacketStatus),
}
