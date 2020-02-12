
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct SelectTradePacket {
    pub item: i32,
}

impl CodablePacket for SelectTradePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.item);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let item = buf.get_mc_var_int()?;
        return Ok(SelectTradePacket { item });
    }
}
