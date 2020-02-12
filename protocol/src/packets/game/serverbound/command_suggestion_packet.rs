
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct CommandSuggestionPacket {
    pub id: i32,
    pub command: String,
}

impl CodablePacket for CommandSuggestionPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.id);
        // TODO: UNKNOWN: var1.writeUtf(this.command, 32500);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let id = buf.get_mc_var_int()?;
        let command = buf.get_mc_string_bounded(32500)?;
        return Ok(CommandSuggestionPacket { id, command });
    }
}
