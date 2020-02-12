
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct TagQueryPacket {
    pub transactionId: i32,
    pub tag: Nbt,
}

impl CodablePacket for TagQueryPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.transactionId);
        buf.set_mc_nbt(self.tag);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let transactionId = buf.get_mc_var_int()?;
        let tag = buf.get_mc_nbt()?;
        return Ok(TagQueryPacket { transactionId, tag });
    }
}
