use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct KeyPacket {
    pub keybytes: Vec<u8>,
    pub nonce: Vec<u8>,
}

impl CodablePacket for KeyPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_byte_array(self.keybytes);
        buf.set_mc_byte_array(self.nonce);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let keybytes = buf.get_mc_byte_array_bounded(2048)?;
        let nonce = buf.get_mc_byte_array_bounded(2048)?;
        return Ok(KeyPacket { keybytes, nonce });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(KeyPacket {
            keybytes: vec![0x1f, 0x2f],
            nonce: vec![0x3f, 0x4f, 0x5f],
        })
    }
}