pub mod hello_packet;
pub use hello_packet::*;
pub mod key_packet;
pub use key_packet::*;
pub mod custom_query_packet;
pub use custom_query_packet::*;

use super::Login;
use crate::network::*;
use crate::packet::*;
use basin2_lib::Result;
use bytes::BytesMut;
use std::io::Error as IoError;
use std::io::ErrorKind;

hierarchy! {
    child<Login> enum LoginServerbound {
        HelloPacket,
        KeyPacket,
        CustomQueryPacket,
    }
}

impl PacketContainer for LoginServerbound {
    fn encode(self, buf: &mut BytesMut) {
        match self {
            LoginServerbound::HelloPacket(deref_packet) => {
                buf.set_mc_var_int(0);
                deref_packet.encode(buf);
            }
            LoginServerbound::KeyPacket(deref_packet) => {
                buf.set_mc_var_int(1);
                deref_packet.encode(buf);
            }
            LoginServerbound::CustomQueryPacket(deref_packet) => {
                buf.set_mc_var_int(2);
                deref_packet.encode(buf);
            }
        }
    }

    fn decode(id: i32, buf: &mut BytesMut) -> Result<LoginServerbound> {
        match id {
            0 => Ok(LoginServerbound::HelloPacket(HelloPacket::decode(buf)?)),
            1 => Ok(LoginServerbound::KeyPacket(KeyPacket::decode(buf)?)),
            2 => Ok(LoginServerbound::CustomQueryPacket(
                CustomQueryPacket::decode(buf)?,
            )),
            _ => Err(Box::new(IoError::from(ErrorKind::InvalidData))),
        }
    }
}
