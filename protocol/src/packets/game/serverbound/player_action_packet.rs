use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub struct PlayerActionPacket {
    pub pos: BlockPos,
    pub direction: Direction,
    pub action: PlayerActionPacketAction,
}

impl CodablePacket for PlayerActionPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.action as i32);
        buf.set_mc_block_pos(self.pos);
        buf.set_mc_u8(self.direction as u8);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let action: PlayerActionPacketAction = buf.get_mc_enum()?;
        let pos = buf.get_mc_block_pos()?;
        let direction: Direction = buf.get_mc_enum_u8()?;
        return Ok(PlayerActionPacket {
            pos,
            direction,
            action,
        });
    }
}
