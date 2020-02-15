use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub struct HelloPacket {
    pub username: String,
}

impl CodablePacket for HelloPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_string(self.username);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let username = buf.get_mc_string(16)?;
        return Ok(HelloPacket { username });
    }
}
