pub mod status_request_packet;
pub use status_request_packet::*;
pub mod ping_request_packet;
pub use ping_request_packet::*;

use crate::network::*;
use crate::packet::*;
use crate::Result;
use bytes::BytesMut;
use std::io::Error as IoError;
use std::io::ErrorKind;

pub fn decode_packet(id: i32, buf: &mut BytesMut) -> Result<PacketStatusServerbound> {
    match id {
        0 => Ok(PacketStatusServerbound::StatusRequestPacket(
            StatusRequestPacket::decode(buf)?,
        )),
        1 => Ok(PacketStatusServerbound::PingRequestPacket(
            PingRequestPacket::decode(buf)?,
        )),
        _ => Err(Box::new(IoError::from(ErrorKind::InvalidData))),
    }
}

pub fn encode_packet(packet: PacketStatusServerbound, buf: &mut BytesMut) {
    match packet {
        PacketStatusServerbound::StatusRequestPacket(deref_packet) => {
            buf.set_mc_var_int(0);
            deref_packet.encode(buf);
        }
        PacketStatusServerbound::PingRequestPacket(deref_packet) => {
            buf.set_mc_var_int(1);
            deref_packet.encode(buf);
        }
    }
}

pub enum PacketStatusServerbound {
    StatusRequestPacket(StatusRequestPacket),
    PingRequestPacket(PingRequestPacket),
}
