
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct ContainerAckPacket {
    pub containerId: i32,
    pub uid: i16,
    pub accepted: bool,
}

impl CodablePacket for ContainerAckPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_u8(self.containerId);
        buf.set_mc_i16(self.uid);
        buf.set_mc_bool(self.accepted);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let containerId = buf.get_mc_u8()?;
        let uid = buf.get_mc_i16()?;
        let accepted = buf.get_mc_bool()?;
        return Ok(ContainerAckPacket { containerId, uid, accepted });
    }
}
