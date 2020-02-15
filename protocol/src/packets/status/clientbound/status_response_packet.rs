
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct StatusResponsePacket {
    pub status: ServerStatus,
}

impl CodablePacket for StatusResponsePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_string(serde_json::to_string(&self.status).unwrap());
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let status: ServerStatus = serde_json::from_str(&*buf.get_mc_string(32767)?)?;
        return Ok(StatusResponsePacket { status });
    }
}
