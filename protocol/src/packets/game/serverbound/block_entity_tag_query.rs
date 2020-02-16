use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct BlockEntityTagQuery {
    pub transactionId: i32,
    pub pos: BlockPos,
}

impl CodablePacket for BlockEntityTagQuery {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.transactionId);
        buf.set_mc_block_pos(self.pos);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let transactionId = buf.get_mc_var_int()?;
        let pos = buf.get_mc_block_pos()?;
        return Ok(BlockEntityTagQuery { transactionId, pos });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(BlockEntityTagQuery {
            transactionId: 1234,
            pos: BlockPos { x: 10, y: 20, z: 30 },
        })
    }
}