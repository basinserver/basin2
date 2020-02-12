
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct CustomQueryPacket {
    pub transactionId: i32,
    pub identifier: ResourceLocation,
    pub data: BytesMut,
}

impl CodablePacket for CustomQueryPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.transactionId);
        buf.set_mc_string(self.identifier);
        // TODO: UNKNOWN: var1.writeBytes(this.data.copy());
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(CustomQueryPacket { transactionId, identifier, data });
    }
}
