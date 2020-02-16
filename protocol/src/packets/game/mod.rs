pub mod serverbound;
pub use serverbound::*;
pub mod clientbound;
pub use clientbound::*;

#[derive(Clone, Debug, PartialEq)]
pub enum PacketGame {
    PacketGameServerbound(PacketGameServerbound),
    PacketGameClientbound(PacketGameClientbound),
}
