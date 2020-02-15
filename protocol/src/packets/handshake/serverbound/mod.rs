pub mod client_intention_packet;
pub use client_intention_packet::*;

use crate::network::*;
use crate::packet::*;
use crate::Result;
use bytes::BytesMut;
use std::io::Error as IoError;
use std::io::ErrorKind;

pub fn decode_packet(id: i32, buf: &mut BytesMut) -> Result<PacketHandshakeServerbound> {
    match id {
        0 => Ok(PacketHandshakeServerbound::ClientIntentionPacket(
            ClientIntentionPacket::decode(buf)?,
        )),
        _ => Err(Box::new(IoError::from(ErrorKind::InvalidData))),
    }
}

pub fn encode_packet(packet: PacketHandshakeServerbound, buf: &mut BytesMut) {
    match packet {
        PacketHandshakeServerbound::ClientIntentionPacket(deref_packet) => {
            buf.set_mc_var_int(0);
            deref_packet.encode(buf);
        }
    }
}

pub enum PacketHandshakeServerbound {
    ClientIntentionPacket(ClientIntentionPacket),
}
