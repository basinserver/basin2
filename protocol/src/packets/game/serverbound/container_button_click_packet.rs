
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct ContainerButtonClickPacket {
    pub containerId: i32,
    pub buttonId: i32,
}

impl CodablePacket for ContainerButtonClickPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_u8(self.containerId);
        buf.set_mc_u8(self.buttonId);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let containerId = buf.get_mc_u8()?;
        let buttonId = buf.get_mc_u8()?;
        return Ok(ContainerButtonClickPacket { containerId, buttonId });
    }
}
