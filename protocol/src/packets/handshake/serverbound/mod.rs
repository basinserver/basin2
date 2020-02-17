pub mod client_intention_packet;
pub use client_intention_packet::*;

use super::Handshake;
use crate::network::*;
use crate::packet::*;
use crate::Result;
use bytes::BytesMut;
use std::io::Error as IoError;
use std::io::ErrorKind;

hierarchy! {
    child<Handshake> enum HandshakeServerbound {
        ClientIntentionPacket,
    }
}

impl PacketContainer for HandshakeServerbound {
    fn encode(self, buf: &mut BytesMut) {
        match self {
            HandshakeServerbound::ClientIntentionPacket(deref_packet) => {
                buf.set_mc_var_int(0);
                deref_packet.encode(buf);
            }
        }
    }

    fn decode(id: i32, buf: &mut BytesMut) -> Result<HandshakeServerbound> {
        match id {
            0 => Ok(HandshakeServerbound::ClientIntentionPacket(
                ClientIntentionPacket::decode(buf)?,
            )),
            _ => Err(Box::new(IoError::from(ErrorKind::InvalidData))),
        }
    }
}
