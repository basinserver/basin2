
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct SetEntityDataPacket {
    pub id: i32,
    pub packedItems: undefined,
}

impl CodablePacket for SetEntityDataPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.id);
        // TODO: EXTRA: SynchedEntityData.pack(this.packedItems, var1);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let id = buf.get_mc_var_int()?;
        // TODO: EXTRA: this.packedItems = SynchedEntityData.unpack(var1);
        return Ok(SetEntityDataPacket { id, packedItems });
    }
}
