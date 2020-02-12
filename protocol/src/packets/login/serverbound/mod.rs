pub mod hello_packet;
pub use hello_packet::*;
pub mod key_packet;
pub use key_packet::*;
pub mod custom_query_packet;
pub use custom_query_packet::*;

use bytes::BytesMut;
use crate::Result;
use std::io::Error as IoError;
use std::io::ErrorKind;
use crate::packet::*;
use crate::network::*;

pub fn decode_packet(id: i32, buf: &mut BytesMut) -> Result<PacketLoginServerbound> {
    match id {
        0 => Ok(PacketLoginServerbound::HelloPacket(HelloPacket::decode(buf)?)),
        1 => Ok(PacketLoginServerbound::KeyPacket(KeyPacket::decode(buf)?)),
        2 => Ok(PacketLoginServerbound::CustomQueryPacket(CustomQueryPacket::decode(buf)?)),
        _ => Err(Box::new(IoError::from(ErrorKind::InvalidData)))
    }
}

pub fn encode_packet(packet: PacketLoginServerbound, buf: &mut BytesMut) {
    match packet {
        PacketLoginServerbound::HelloPacket(deref_packet) => { buf.set_mc_var_int(0); deref_packet.encode(buf); },
        PacketLoginServerbound::KeyPacket(deref_packet) => { buf.set_mc_var_int(1); deref_packet.encode(buf); },
        PacketLoginServerbound::CustomQueryPacket(deref_packet) => { buf.set_mc_var_int(2); deref_packet.encode(buf); },
    }
}

pub enum PacketLoginServerbound {
    HelloPacket(HelloPacket),
    KeyPacket(KeyPacket),
    CustomQueryPacket(CustomQueryPacket),
}

