pub mod serverbound;
pub use serverbound::*;
pub mod clientbound;
pub use clientbound::*;

pub enum PacketLogin {
    PacketLoginServerbound(PacketLoginServerbound),
    PacketLoginClientbound(PacketLoginClientbound),
}
