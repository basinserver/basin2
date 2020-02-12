pub mod serverbound;
pub use serverbound::*;
pub mod clientbound;
pub use clientbound::*;

pub enum PacketHandshake {
    PacketHandshakeServerbound(PacketHandshakeServerbound),
    PacketHandshakeClientbound(PacketHandshakeClientbound),
}
