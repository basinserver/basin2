
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct ResourcePackPacket {
    pub action: ResourcePackPacketAction,
}

impl CodablePacket for ResourcePackPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.action as i32);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let action: ResourcePackPacketAction = buf.get_mc_enum()?;
        return Ok(ResourcePackPacket { action });
    }
}
