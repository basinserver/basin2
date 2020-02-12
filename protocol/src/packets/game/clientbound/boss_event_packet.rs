
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct BossEventPacket {
    pub id: Uuid,
    pub operation: BossEventPacketOperation,
    pub name: ChatComponent,
    pub pct: f32,
    pub color: BossBarColor,
    pub overlay: BossBarOverlay,
    pub darkenScreen: bool,
    pub playMusic: bool,
    pub createWorldFog: bool,
}

impl CodablePacket for BossEventPacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        /* TODO: NOT FOUND */
        return Ok(BossEventPacket { id, operation, name, pct, color, overlay, darkenScreen, playMusic, createWorldFog });
    }
}
