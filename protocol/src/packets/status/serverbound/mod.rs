pub mod status_request_packet;
pub use status_request_packet::*;
pub mod ping_request_packet;
pub use ping_request_packet::*;

use super::Status;
use crate::network::*;
use crate::packet::*;
use basin2_lib::Result;
use bytes::BytesMut;
use std::io::Error as IoError;
use std::io::ErrorKind;

hierarchy! {
    child<Status> enum StatusServerbound {
        StatusRequestPacket,
        PingRequestPacket,
    }
}

impl PacketContainer for StatusServerbound {
    fn encode(self, buf: &mut BytesMut) {
        match self {
            StatusServerbound::StatusRequestPacket(deref_packet) => {
                buf.set_mc_var_int(0);
                deref_packet.encode(buf);
            }
            StatusServerbound::PingRequestPacket(deref_packet) => {
                buf.set_mc_var_int(1);
                deref_packet.encode(buf);
            }
        }
    }

    fn decode(id: i32, buf: &mut BytesMut) -> Result<StatusServerbound> {
        match id {
            0 => Ok(StatusServerbound::StatusRequestPacket(
                StatusRequestPacket::decode(buf)?,
            )),
            1 => Ok(StatusServerbound::PingRequestPacket(
                PingRequestPacket::decode(buf)?,
            )),
            _ => Err(Box::new(IoError::from(ErrorKind::InvalidData))),
        }
    }
}
