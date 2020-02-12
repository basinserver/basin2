
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct BlockBreakAckPacket {
    pub pos: BlockPos,
    pub state: BlockState,
    pub allGood: bool,
}

impl CodablePacket for BlockBreakAckPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_block_pos(self.pos);
        // TODO: UNKNOWN: var1.writeVarInt(Block.getId(this.state));
        buf.set_mc_var_int(self.action);
        buf.set_mc_bool(self.allGood);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let pos = buf.get_mc_block_pos()?;
        // TODO: UNKNOWN: this.state = (BlockState)Block.BLOCK_STATE_REGISTRY.byId(var1.readVarInt());
        // TODO: UNKNOWN: this.action = (ServerboundPlayerActionPacket.Action)var1.readEnum(ServerboundPlayerActionPacket.Action.class);
        let allGood = buf.get_mc_bool()?;
        return Ok(BlockBreakAckPacket { pos, state, allGood });
    }
}
