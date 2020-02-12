
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct BlockUpdatePacket {
    pub pos: BlockPos,
    pub blockState: BlockState,
}

impl CodablePacket for BlockUpdatePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_block_pos(self.pos);
        // TODO: UNKNOWN: var1.writeVarInt(Block.getId(this.blockState));
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let pos = buf.get_mc_block_pos()?;
        // TODO: UNKNOWN: this.blockState = (BlockState)Block.BLOCK_STATE_REGISTRY.byId(var1.readVarInt());
        return Ok(BlockUpdatePacket { pos, blockState });
    }
}
