use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct BlockBreakAckPacket {
    pub pos: BlockPos,
    pub state: BlockState,
    pub action: PlayerActionPacketAction,
    pub allGood: bool,
}

impl CodablePacket for BlockBreakAckPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_block_pos(self.pos);
        buf.set_mc_var_int(self.state);
        buf.set_mc_var_int(self.action as i32);
        buf.set_mc_bool(self.allGood);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let pos = buf.get_mc_block_pos()?;
        let state = buf.get_mc_var_int()?;
        let action: PlayerActionPacketAction = buf.get_mc_enum()?;
        let allGood = buf.get_mc_bool()?;
        return Ok(BlockBreakAckPacket {
            pos,
            state,
            action,
            allGood,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(BlockBreakAckPacket {
            pos: BlockPos {
                x: -100,
                y: 12,
                z: 1024,
            },
            state: 6432,
            action: PlayerActionPacketAction::StartDestroyBlock,
            allGood: true,
        })
    }
}
