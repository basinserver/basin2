
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct HorseScreenOpenPacket {
    pub containerId: i32,
    pub size: i32,
    pub entityId: i32,
}

impl CodablePacket for HorseScreenOpenPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_u8(self.containerId);
        buf.set_mc_var_int(self.size);
        buf.set_mc_i32(self.entityId);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let containerId = buf.get_mc_u8()?;
        let size = buf.get_mc_var_int()?;
        let entityId = buf.get_mc_i32()?;
        return Ok(HorseScreenOpenPacket { containerId, size, entityId });
    }
}
