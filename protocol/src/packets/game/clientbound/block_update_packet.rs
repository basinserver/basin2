use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct BlockUpdatePacket {
    pub pos: BlockPos,
    pub blockState: BlockState,
}

impl CodablePacket for BlockUpdatePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_block_pos(self.pos);
        buf.set_mc_var_int(self.blockState);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let pos = buf.get_mc_block_pos()?;
        let blockState = buf.get_mc_var_int()?;
        return Ok(BlockUpdatePacket { pos, blockState });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(BlockUpdatePacket {
            pos: BlockPos {
                x: -100,
                y: 12,
                z: 1024,
            },
            blockState: 453242,
        })
    }
}
