
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct PlayerCombatPacket {
    pub event: PlayerCombatPacketEvent,
    pub playerId: i32,
    pub killerId: i32,
    pub duration: i32,
    pub message: ChatComponent,
}

impl CodablePacket for PlayerCombatPacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(PlayerCombatPacket { event, playerId, killerId, duration, message });
    }
}
