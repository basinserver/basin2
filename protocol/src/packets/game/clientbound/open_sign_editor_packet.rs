use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct OpenSignEditorPacket {
    pub pos: BlockPos,
}

impl CodablePacket for OpenSignEditorPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_block_pos(self.pos);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let pos = buf.get_mc_block_pos()?;
        return Ok(OpenSignEditorPacket { pos });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(OpenSignEditorPacket {
            pos: BlockPos { x: 34, y: 12, z: -56343 },
        })
    }
}