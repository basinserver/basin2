use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct SignUpdatePacket {
    pub pos: BlockPos,
    pub lines: (String, String, String, String),
}

impl CodablePacket for SignUpdatePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_block_pos(self.pos);
        buf.set_mc_string(self.lines.0);
        buf.set_mc_string(self.lines.1);
        buf.set_mc_string(self.lines.2);
        buf.set_mc_string(self.lines.3);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let pos = buf.get_mc_block_pos()?;
        let lines = (
            buf.get_mc_string(384)?,
            buf.get_mc_string(384)?,
            buf.get_mc_string(384)?,
            buf.get_mc_string(384)?,
        );
        return Ok(SignUpdatePacket { pos, lines });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(SignUpdatePacket {
            pos: BlockPos { x: -10, y: 30, z: -30 },
            lines: ("line1".to_string(), "line2".to_string(), "line3".to_string(), "line4".to_string()),
        })
    }
}