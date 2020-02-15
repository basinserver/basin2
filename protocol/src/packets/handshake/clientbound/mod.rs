
use bytes::BytesMut;
use crate::Result;
use std::io::Error as IoError;
use std::io::ErrorKind;

pub fn decode_packet(id: i32, buf: &mut BytesMut) -> Result<PacketHandshakeClientbound> {
    match id {
        
        _ => Err(Box::new(IoError::from(ErrorKind::InvalidData)))
    }
}

pub fn encode_packet(packet: PacketHandshakeClientbound, buf: &mut BytesMut) {
    match packet {
        
    }
}

pub enum PacketHandshakeClientbound {
    
}

