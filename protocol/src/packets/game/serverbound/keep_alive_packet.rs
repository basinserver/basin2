use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub struct KeepAlivePacket {
    pub id: i64,
}

impl CodablePacket for KeepAlivePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_i64(self.id);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let id = buf.get_mc_i64()?;
        return Ok(KeepAlivePacket { id });
    }
}
