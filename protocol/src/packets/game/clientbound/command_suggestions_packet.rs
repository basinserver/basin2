
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct CommandSuggestionsPacket {
    pub id: i32,
    pub suggestions: undefined,
}

impl CodablePacket for CommandSuggestionsPacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(CommandSuggestionsPacket { id, suggestions });
    }
}
