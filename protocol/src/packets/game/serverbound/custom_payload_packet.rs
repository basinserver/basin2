
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct CustomPayloadPacket {
    pub identifier: ResourceLocation,
    pub data: BytesMut,
}

impl CodablePacket for CustomPayloadPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_string(self.identifier);
        // TODO: UNKNOWN: var1.writeBytes((ByteBuf)this.data);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(CustomPayloadPacket { identifier, data });
    }
}
