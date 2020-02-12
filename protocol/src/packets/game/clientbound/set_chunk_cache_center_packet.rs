
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct SetChunkCacheCenterPacket {
    pub x: i32,
    pub z: i32,
}

impl CodablePacket for SetChunkCacheCenterPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.x);
        buf.set_mc_var_int(self.z);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let x = buf.get_mc_var_int()?;
        let z = buf.get_mc_var_int()?;
        return Ok(SetChunkCacheCenterPacket { x, z });
    }
}
