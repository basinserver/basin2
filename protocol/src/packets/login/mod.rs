pub mod serverbound;
pub use serverbound::*;
pub mod clientbound;
pub use clientbound::*;

#[derive(Clone, Debug, PartialEq)]
pub enum PacketLogin {
    PacketLoginServerbound(PacketLoginServerbound),
    PacketLoginClientbound(PacketLoginClientbound),
}
