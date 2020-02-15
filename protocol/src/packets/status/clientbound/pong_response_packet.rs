
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct PongResponsePacket {
    pub time: i64,
}

impl CodablePacket for PongResponsePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_i64(self.time);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let time = buf.get_mc_i64()?;
        return Ok(PongResponsePacket { time });
    }
}
