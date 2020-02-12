
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct EntityTagQuery {
    pub transactionId: i32,
    pub entityId: i32,
}

impl CodablePacket for EntityTagQuery {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.transactionId);
        buf.set_mc_var_int(self.entityId);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let transactionId = buf.get_mc_var_int()?;
        let entityId = buf.get_mc_var_int()?;
        return Ok(EntityTagQuery { transactionId, entityId });
    }
}
