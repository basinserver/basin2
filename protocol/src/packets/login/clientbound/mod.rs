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

use crate::network::*;
use crate::packet::*;
use crate::Result;
use bytes::BytesMut;
use std::io::Error as IoError;
use std::io::ErrorKind;

#[derive(Clone, Debug, PartialEq)]
pub enum PacketLoginClientbound {
    LoginDisconnectPacket(LoginDisconnectPacket),
    HelloPacket(HelloPacket),
    GameProfilePacket(GameProfilePacket),
    LoginCompressionPacket(LoginCompressionPacket),
    CustomQueryPacket(CustomQueryPacket),
}

impl PacketContainer for PacketLoginClientbound {
    fn encode(self, buf: &mut BytesMut) {
        match self {
            PacketLoginClientbound::LoginDisconnectPacket(deref_packet) => {
                buf.set_mc_var_int(0);
                deref_packet.encode(buf);
            }
            PacketLoginClientbound::HelloPacket(deref_packet) => {
                buf.set_mc_var_int(1);
                deref_packet.encode(buf);
            }
            PacketLoginClientbound::GameProfilePacket(deref_packet) => {
                buf.set_mc_var_int(2);
                deref_packet.encode(buf);
            }
            PacketLoginClientbound::LoginCompressionPacket(deref_packet) => {
                buf.set_mc_var_int(3);
                deref_packet.encode(buf);
            }
            PacketLoginClientbound::CustomQueryPacket(deref_packet) => {
                buf.set_mc_var_int(4);
                deref_packet.encode(buf);
            }
        }
    }

    fn decode(id: i32, buf: &mut BytesMut) -> Result<PacketLoginClientbound> {
        match id {
            0 => Ok(PacketLoginClientbound::LoginDisconnectPacket(
                LoginDisconnectPacket::decode(buf)?,
            )),
            1 => Ok(PacketLoginClientbound::HelloPacket(HelloPacket::decode(
                buf,
            )?)),
            2 => Ok(PacketLoginClientbound::GameProfilePacket(
                GameProfilePacket::decode(buf)?,
            )),
            3 => Ok(PacketLoginClientbound::LoginCompressionPacket(
                LoginCompressionPacket::decode(buf)?,
            )),
            4 => Ok(PacketLoginClientbound::CustomQueryPacket(
                CustomQueryPacket::decode(buf)?,
            )),
            _ => Err(Box::new(IoError::from(ErrorKind::InvalidData))),
        }
    }
}
