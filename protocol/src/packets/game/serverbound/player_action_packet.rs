
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct PlayerActionPacket {
    pub pos: BlockPos,
    pub direction: Direction,
    pub action: PlayerActionPacketAction,
}

impl CodablePacket for PlayerActionPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.action);
        buf.set_mc_block_pos(self.pos);
        // TODO: UNKNOWN: var1.writeByte(this.direction.get3DDataValue());
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        // TODO: UNKNOWN: this.action = (ServerboundPlayerActionPacket.Action)var1.readEnum(ServerboundPlayerActionPacket.Action.class);
        let pos = buf.get_mc_block_pos()?;
        // TODO: UNKNOWN: this.direction = Direction.from3DDataValue(var1.readUnsignedByte());
        return Ok(PlayerActionPacket { pos, direction, action });
    }
}
