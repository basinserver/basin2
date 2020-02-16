pub mod serverbound;
pub use serverbound::*;
pub mod clientbound;
pub use clientbound::*;

#[derive(Clone, Debug, PartialEq)]
pub enum PacketStatus {
    PacketStatusServerbound(PacketStatusServerbound),
    PacketStatusClientbound(PacketStatusClientbound),
}
