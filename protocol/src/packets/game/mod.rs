pub mod serverbound;
pub use serverbound::*;
pub mod clientbound;
pub use clientbound::*;

pub enum PacketGame {
    PacketGameServerbound(PacketGameServerbound),
    PacketGameClientbound(PacketGameClientbound),
}
