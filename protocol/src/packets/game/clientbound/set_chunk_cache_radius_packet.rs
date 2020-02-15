
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct SetChunkCacheRadiusPacket {
    pub radius: i32,
}

impl CodablePacket for SetChunkCacheRadiusPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.radius);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let radius = buf.get_mc_var_int()?;
        return Ok(SetChunkCacheRadiusPacket { radius });
    }
}
