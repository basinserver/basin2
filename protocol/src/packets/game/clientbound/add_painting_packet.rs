use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;
use uuid::Uuid;

#[derive(PartialEq, Clone, Debug)]
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
        buf.set_mc_u8(self.direction.get_2d_direction());
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let id = buf.get_mc_var_int()?;
        let uuid = buf.get_mc_uuid()?;
        let motive = buf.get_mc_var_int()?;
        let pos = buf.get_mc_block_pos()?;
        let direction = Direction::from_2d_direction(buf.get_mc_u8()?).unwrap_or(Direction::Up);
        return Ok(AddPaintingPacket {
            id,
            uuid,
            pos,
            direction,
            motive,
        });
    }
}
