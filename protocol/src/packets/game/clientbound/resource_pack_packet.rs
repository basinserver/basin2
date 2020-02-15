
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct ResourcePackPacket {
    pub url: String,
    pub hash: String,
}

impl CodablePacket for ResourcePackPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_string(self.url);
        buf.set_mc_string(self.hash);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let url = buf.get_mc_string(32767)?;
        let hash = buf.get_mc_string(40)?;
        return Ok(ResourcePackPacket { url, hash });
    }
}
