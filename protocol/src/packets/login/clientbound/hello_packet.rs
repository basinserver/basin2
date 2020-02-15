use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub struct HelloPacket {
    pub serverId: String,
    pub publicKey: Vec<u8>,
    pub nonce: Vec<u8>,
}

impl CodablePacket for HelloPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_string(self.serverId);
        buf.set_mc_byte_array(self.publicKey);
        buf.set_mc_byte_array(self.nonce);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let serverId = buf.get_mc_string(20)?;
        let publicKey = buf.get_mc_byte_array()?;
        let nonce = buf.get_mc_byte_array()?;
        return Ok(HelloPacket {
            serverId,
            publicKey,
            nonce,
        });
    }
}
