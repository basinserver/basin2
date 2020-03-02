use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct LevelEventPacket {
    pub eventType: i32,
    pub pos: BlockPos,
    pub data: i32,
    pub globalEvent: bool,
}

impl CodablePacket for LevelEventPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_i32(self.eventType);
        buf.set_mc_block_pos(self.pos);
        buf.set_mc_i32(self.data);
        buf.set_mc_bool(self.globalEvent);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let eventType = buf.get_mc_i32()?;
        let pos = buf.get_mc_block_pos()?;
        let data = buf.get_mc_i32()?;
        let globalEvent = buf.get_mc_bool()?;
        return Ok(LevelEventPacket {
            eventType,
            pos,
            data,
            globalEvent,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(LevelEventPacket {
            eventType: 12,
            pos: BlockPos {
                x: 123,
                y: 20,
                z: -100000,
            },
            data: 4535345,
            globalEvent: true,
        })
    }
}
