use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct BlockDestructionPacket {
    pub id: i32,
    pub pos: BlockPos,
    pub progress: u8,
}

impl CodablePacket for BlockDestructionPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.id);
        buf.set_mc_block_pos(self.pos);
        buf.set_mc_u8(self.progress);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let id = buf.get_mc_var_int()?;
        let pos = buf.get_mc_block_pos()?;
        let progress = buf.get_mc_u8()?;
        return Ok(BlockDestructionPacket { id, pos, progress });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(BlockDestructionPacket {
            id: 354343,
            pos: BlockPos {
                x: -100,
                y: 12,
                z: 1024,
            },
            progress: 12,
        })
    }
}
