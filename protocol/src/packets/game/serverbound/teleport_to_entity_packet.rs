
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct TeleportToEntityPacket {
    pub uuid: Uuid,
}

impl CodablePacket for TeleportToEntityPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_uuid(self.uuid);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let uuid = buf.get_mc_uuid()?;
        return Ok(TeleportToEntityPacket { uuid });
    }
}
