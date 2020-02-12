
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

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
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let pos = buf.get_mc_block_pos()?;
        let command = buf.get_mc_string_bounded(32767)?;
        // TODO: UNKNOWN: this.mode = (CommandBlockEntity.Mode)var1.readEnum(CommandBlockEntity.Mode.class);
        // TODO: UNKNOWN: byte var2 = var1.readByte();
        // TODO: EXTRA: this.trackOutput = (var2 & 1) != 0;
        // TODO: EXTRA: this.conditional = (var2 & 2) != 0;
        // TODO: EXTRA: this.automatic = (var2 & 4) != 0;
        return Ok(SetCommandBlockPacket { pos, command, trackOutput, conditional, automatic, mode });
    }
}
