use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct SetCommandBlockPacket {
    pub pos: BlockPos,
    pub command: String,
    pub trackOutput: bool,
    pub conditional: bool,
    pub automatic: bool,
    pub mode: CommandBlockEntityMode,
}

impl CodablePacket for SetCommandBlockPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_block_pos(self.pos);
        buf.set_mc_string(self.command);
        buf.set_mc_var_int(self.mode as i32);
        let mut flags = 0;
        if self.trackOutput {
            flags |= 1;
        }
        if self.conditional {
            flags |= 2;
        }
        if self.automatic {
            flags |= 4;
        }
        buf.set_mc_u8(flags);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let pos = buf.get_mc_block_pos()?;
        let command = buf.get_mc_string(32767)?;
        let mode: CommandBlockEntityMode = buf.get_mc_enum()?;
        let flags = buf.get_mc_u8()?;
        let trackOutput = (flags & 1) > 0;
        let conditional = (flags & 2) > 0;
        let automatic = (flags & 4) > 0;
        return Ok(SetCommandBlockPacket {
            pos,
            command,
            trackOutput,
            conditional,
            automatic,
            mode,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(SetCommandBlockPacket {
            pos: BlockPos {
                x: -10,
                y: -20,
                z: -30,
            },
            command: "do stuff".to_string(),
            trackOutput: true,
            conditional: false,
            automatic: true,
            mode: CommandBlockEntityMode::Redstone,
        })
    }
}
