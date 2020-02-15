use crate::packet::*;
use crate::Result;
use bytes::BytesMut;
use std::io::Error as IoError;
use std::io::ErrorKind;

pub enum PacketHandshakeClientbound {}

impl PacketContainer for PacketHandshakeClientbound {
    fn encode(self, _buf: &mut BytesMut) {
        match self {}
    }

    fn decode(id: i32, _buf: &mut BytesMut) -> Result<PacketHandshakeClientbound> {
        match id {
            _ => Err(Box::new(IoError::from(ErrorKind::InvalidData))),
        }
    }
}
