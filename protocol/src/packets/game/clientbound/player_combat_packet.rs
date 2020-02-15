
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct PlayerCombatPacket {
    pub event: PlayerCombatPacketEvent,
    pub playerId: i32,
    pub killerId: i32,
    pub duration: i32,
    pub message: Option<ChatComponent>,
}

impl CodablePacket for PlayerCombatPacket {
    fn encode(self, buf: &mut BytesMut) {
        use PlayerCombatPacketEvent::*;

        buf.set_mc_var_int(self.event as i32);
        match self.event {
            EndCombat => {
                buf.set_mc_var_int(self.duration);
                buf.set_mc_i32(self.killerId);
            },
            EntityDied => {
                buf.set_mc_var_int(self.playerId);
                buf.set_mc_i32(self.killerId);
                buf.set_mc_chat_component(self.message.unwrap_or("".to_string()));
            },
            _ => (),
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        use PlayerCombatPacketEvent::*;

        let event: PlayerCombatPacketEvent = buf.get_mc_enum()?;
        let (playerId, killerId, duration, message) =
        match event {
            EndCombat => {(
                0,
                buf.get_mc_i32()?,
                buf.get_mc_var_int()?,
                None,
            )},
            EntityDied => {(
                buf.get_mc_var_int()?,
                buf.get_mc_i32()?,
                0,
                Some(buf.get_mc_chat_component()?),
            )},
            EnterCombat => (0, 0, 0, None),
        };
        return Ok(PlayerCombatPacket { event, playerId, killerId, duration, message });
    }
}
