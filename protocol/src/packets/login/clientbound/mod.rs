pub mod login_disconnect_packet;
pub use login_disconnect_packet::*;
pub mod hello_packet;
pub use hello_packet::*;
pub mod game_profile_packet;
pub use game_profile_packet::*;
pub mod login_compression_packet;
pub use login_compression_packet::*;
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
    child<Login> enum LoginClientbound {
        LoginDisconnectPacket,
        HelloPacket,
        GameProfilePacket,
        LoginCompressionPacket,
        CustomQueryPacket,
    }
}

impl PacketContainer for LoginClientbound {
    fn encode(self, buf: &mut BytesMut) {
        match self {
            LoginClientbound::LoginDisconnectPacket(deref_packet) => {
                buf.set_mc_var_int(0);
                deref_packet.encode(buf);
            }
            LoginClientbound::HelloPacket(deref_packet) => {
                buf.set_mc_var_int(1);
                deref_packet.encode(buf);
            }
            LoginClientbound::GameProfilePacket(deref_packet) => {
                buf.set_mc_var_int(2);
                deref_packet.encode(buf);
            }
            LoginClientbound::LoginCompressionPacket(deref_packet) => {
                buf.set_mc_var_int(3);
                deref_packet.encode(buf);
            }
            LoginClientbound::CustomQueryPacket(deref_packet) => {
                buf.set_mc_var_int(4);
                deref_packet.encode(buf);
            }
        }
    }

    fn decode(id: i32, buf: &mut BytesMut) -> Result<LoginClientbound> {
        match id {
            0 => Ok(LoginClientbound::LoginDisconnectPacket(
                LoginDisconnectPacket::decode(buf)?,
            )),
            1 => Ok(LoginClientbound::HelloPacket(HelloPacket::decode(buf)?)),
            2 => Ok(LoginClientbound::GameProfilePacket(
                GameProfilePacket::decode(buf)?,
            )),
            3 => Ok(LoginClientbound::LoginCompressionPacket(
                LoginCompressionPacket::decode(buf)?,
            )),
            4 => Ok(LoginClientbound::CustomQueryPacket(
                CustomQueryPacket::decode(buf)?,
            )),
            _ => Err(Box::new(IoError::from(ErrorKind::InvalidData))),
        }
    }
}
