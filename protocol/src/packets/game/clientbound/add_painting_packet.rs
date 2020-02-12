
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct AddPaintingPacket {
    pub id: i32,
    pub uuid: Uuid,
    pub pos: BlockPos,
    pub direction: Direction,
    pub motive: i32,
}

impl CodablePacket for AddPaintingPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.id);
        buf.set_mc_uuid(self.uuid);
        buf.set_mc_var_int(self.motive);
        buf.set_mc_block_pos(self.pos);
        // TODO: UNKNOWN: var1.writeByte(this.direction.get2DDataValue());
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let id = buf.get_mc_var_int()?;
        let uuid = buf.get_mc_uuid()?;
        let motive = buf.get_mc_var_int()?;
        let pos = buf.get_mc_block_pos()?;
        // TODO: UNKNOWN: this.direction = Direction.from2DDataValue(var1.readUnsignedByte());
        return Ok(AddPaintingPacket { id, uuid, pos, direction, motive });
    }
}
