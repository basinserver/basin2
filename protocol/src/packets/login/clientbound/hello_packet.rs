use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
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
        let publicKey = buf.get_mc_byte_array_bounded(2048)?;
        let nonce = buf.get_mc_byte_array_bounded(2048)?;
        return Ok(HelloPacket {
            serverId,
            publicKey,
            nonce,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(HelloPacket {
            serverId: "test".to_string(),
            publicKey: vec![0x0a, 0x0b],
            nonce: "theNonce".as_bytes().to_vec(),
        })
    }
}
