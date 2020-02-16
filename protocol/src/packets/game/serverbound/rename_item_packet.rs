use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct RenameItemPacket {
    pub name: String,
}

impl CodablePacket for RenameItemPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_string(self.name);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let name = buf.get_mc_string(32767)?;
        return Ok(RenameItemPacket { name });
    }
}
