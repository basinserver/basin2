pub mod status_response_packet;
pub use status_response_packet::*;
pub mod pong_response_packet;
pub use pong_response_packet::*;

use crate::network::*;
use crate::packet::*;
use crate::Result;
use bytes::BytesMut;
use std::io::Error as IoError;
use std::io::ErrorKind;

pub fn decode_packet(id: i32, buf: &mut BytesMut) -> Result<PacketStatusClientbound> {
    match id {
        0 => Ok(PacketStatusClientbound::StatusResponsePacket(
            StatusResponsePacket::decode(buf)?,
        )),
        1 => Ok(PacketStatusClientbound::PongResponsePacket(
            PongResponsePacket::decode(buf)?,
        )),
        _ => Err(Box::new(IoError::from(ErrorKind::InvalidData))),
    }
}

pub fn encode_packet(packet: PacketStatusClientbound, buf: &mut BytesMut) {
    match packet {
        PacketStatusClientbound::StatusResponsePacket(deref_packet) => {
            buf.set_mc_var_int(0);
            deref_packet.encode(buf);
        }
        PacketStatusClientbound::PongResponsePacket(deref_packet) => {
            buf.set_mc_var_int(1);
            deref_packet.encode(buf);
        }
    }
}

pub enum PacketStatusClientbound {
    StatusResponsePacket(StatusResponsePacket),
    PongResponsePacket(PongResponsePacket),
}
