
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct HelloPacket {
    pub serverId: String,
    pub publicKey: Vec<u8>,
    pub nonce: Vec<u8>,
}

impl CodablePacket for HelloPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_string(self.serverId);
        // TODO: UNKNOWN: var1.writeByteArray(this.publicKey.getEncoded());
        buf.set_mc_byte_array(self.nonce);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let serverId = buf.get_mc_string_bounded(20)?;
        // TODO: UNKNOWN: this.publicKey = Crypt.byteToPublicKey(var1.readByteArray());
        let nonce = buf.get_mc_byte_array()?;
        return Ok(HelloPacket { serverId, publicKey, nonce });
    }
}
