
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct ResourcePackPacket {
    pub action: ResourcePackPacketAction,
}

impl CodablePacket for ResourcePackPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.action);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        // TODO: UNKNOWN: this.action = (ServerboundResourcePackPacket.Action)var1.readEnum(ServerboundResourcePackPacket.Action.class);
        return Ok(ResourcePackPacket { action });
    }
}
