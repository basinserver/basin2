
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct ForgetLevelChunkPacket {
    pub x: i32,
    pub z: i32,
}

impl CodablePacket for ForgetLevelChunkPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_i32(self.x);
        buf.set_mc_i32(self.z);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let x = buf.get_mc_i32()?;
        let z = buf.get_mc_i32()?;
        return Ok(ForgetLevelChunkPacket { x, z });
    }
}
