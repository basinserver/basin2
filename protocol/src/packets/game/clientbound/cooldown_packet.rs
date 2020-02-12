
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct CooldownPacket {
    pub item: Item,
    pub duration: i32,
}

impl CodablePacket for CooldownPacket {
    fn encode(self, buf: &mut BytesMut) {
        // TODO: UNKNOWN: var1.writeVarInt(Item.getId(this.item));
        buf.set_mc_var_int(self.duration);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        // TODO: UNKNOWN: this.item = Item.byId(var1.readVarInt());
        let duration = buf.get_mc_var_int()?;
        return Ok(CooldownPacket { item, duration });
    }
}
