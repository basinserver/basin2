use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct CooldownPacket {
    pub item: Item,
    pub duration: i32,
}

impl CodablePacket for CooldownPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.item);
        buf.set_mc_var_int(self.duration);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let item = buf.get_mc_var_int()?;
        let duration = buf.get_mc_var_int()?;
        return Ok(CooldownPacket { item, duration });
    }
}
