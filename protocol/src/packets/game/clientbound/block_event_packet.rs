use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct BlockEventPacket {
    pub pos: BlockPos,
    pub b0: u8,
    pub b1: u8,
    pub block: Block,
}

impl CodablePacket for BlockEventPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_block_pos(self.pos);
        buf.set_mc_u8(self.b0);
        buf.set_mc_u8(self.b1);
        buf.set_mc_var_int(self.block);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let pos = buf.get_mc_block_pos()?;
        let b0 = buf.get_mc_u8()?;
        let b1 = buf.get_mc_u8()?;
        let block = buf.get_mc_var_int()?;
        return Ok(BlockEventPacket { pos, b0, b1, block });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(BlockEventPacket {
            pos: BlockPos {
                x: -100,
                y: 12,
                z: 1024,
            },
            b0: 12,
            b1: 24,
            block: 453242,
        })
    }
}
