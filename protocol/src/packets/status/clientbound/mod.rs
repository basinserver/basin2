pub mod status_response_packet;
pub use status_response_packet::*;
pub mod pong_response_packet;
pub use pong_response_packet::*;

use super::Status;
use crate::network::*;
use crate::packet::*;
use basin2_lib::Result;
use bytes::BytesMut;
use std::io::Error as IoError;
use std::io::ErrorKind;

hierarchy! {
    child<Status> enum StatusClientbound {
        StatusResponsePacket,
        PongResponsePacket,
    }
}

impl PacketContainer for StatusClientbound {
    fn encode(self, buf: &mut BytesMut) {
        match self {
            StatusClientbound::StatusResponsePacket(deref_packet) => {
                buf.set_mc_var_int(0);
                deref_packet.encode(buf);
            }
            StatusClientbound::PongResponsePacket(deref_packet) => {
                buf.set_mc_var_int(1);
                deref_packet.encode(buf);
            }
        }
    }

    fn decode(id: i32, buf: &mut BytesMut) -> Result<StatusClientbound> {
        match id {
            0 => Ok(StatusClientbound::StatusResponsePacket(
                StatusResponsePacket::decode(buf)?,
            )),
            1 => Ok(StatusClientbound::PongResponsePacket(
                PongResponsePacket::decode(buf)?,
            )),
            _ => Err(Box::new(IoError::from(ErrorKind::InvalidData))),
        }
    }
}
