pub mod serverbound;
pub use serverbound::*;
pub mod clientbound;
pub use clientbound::*;

pub enum PacketStatus {
    PacketStatusServerbound(PacketStatusServerbound),
    PacketStatusClientbound(PacketStatusClientbound),
}
