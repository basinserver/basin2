
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct ContainerButtonClickPacket {
    pub containerId: u8,
    pub buttonId: u8,
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
