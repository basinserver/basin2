
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct SetTimePacket {
    pub gameTime: i64,
    pub dayTime: i64,
}

impl CodablePacket for SetTimePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_i64(self.gameTime);
        buf.set_mc_i64(self.dayTime);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let gameTime = buf.get_mc_i64()?;
        let dayTime = buf.get_mc_i64()?;
        return Ok(SetTimePacket { gameTime, dayTime });
    }
}
