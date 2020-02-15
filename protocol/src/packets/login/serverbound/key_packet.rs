
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct KeyPacket {
    pub keybytes: Vec<u8>,
    pub nonce: Vec<u8>,
}

impl CodablePacket for KeyPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_byte_array(self.keybytes);
        buf.set_mc_byte_array(self.nonce);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let keybytes = buf.get_mc_byte_array()?;
        let nonce = buf.get_mc_byte_array()?;
        return Ok(KeyPacket { keybytes, nonce });
    }
}
