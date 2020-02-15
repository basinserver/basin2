
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct SetDisplayObjectivePacket {
    pub slot: u8,
    pub objectiveName: String,
}

impl CodablePacket for SetDisplayObjectivePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_u8(self.slot);
        buf.set_mc_string(self.objectiveName);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let slot = buf.get_mc_u8()?;
        let objectiveName = buf.get_mc_string(16)?;
        return Ok(SetDisplayObjectivePacket { slot, objectiveName });
    }
}
