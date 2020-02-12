
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct SetPlayerTeamPacket {
    pub name: String,
    pub displayName: ChatComponent,
    pub playerPrefix: ChatComponent,
    pub playerSuffix: ChatComponent,
    pub nametagVisibility: String,
    pub collisionRule: String,
    pub color: ChatFormatting,
    pub method: i32,
    pub options: i32,
}

impl CodablePacket for SetPlayerTeamPacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(SetPlayerTeamPacket { name, displayName, playerPrefix, playerSuffix, nametagVisibility, collisionRule, color, method, options });
    }
}
