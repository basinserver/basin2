use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
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
            }
            EntityDied => {
                buf.set_mc_var_int(self.playerId);
                buf.set_mc_i32(self.killerId);
                buf.set_mc_chat_component(
                    self.message.unwrap_or(ChatComponent::from("".to_string())),
                );
            }
            _ => (),
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        use PlayerCombatPacketEvent::*;

        let event: PlayerCombatPacketEvent = buf.get_mc_enum()?;
        let (playerId, killerId, duration, message) = match event {
            EndCombat => {
                let duration = buf.get_mc_var_int()?;
                let killerId = buf.get_mc_i32()?;
                (0, killerId, duration, None)
            }
            EntityDied => (
                buf.get_mc_var_int()?,
                buf.get_mc_i32()?,
                0,
                Some(buf.get_mc_chat_component()?),
            ),
            EnterCombat => (0, 0, 0, None),
        };
        return Ok(PlayerCombatPacket {
            event,
            playerId,
            killerId,
            duration,
            message,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle_end() -> Result<()> {
        cycle(PlayerCombatPacket {
            event: PlayerCombatPacketEvent::EndCombat,
            playerId: 0,
            killerId: 3453,
            duration: 65435,
            message: None,
        })
    }

    #[test]
    fn test_cycle_died() -> Result<()> {
        cycle(PlayerCombatPacket {
            event: PlayerCombatPacketEvent::EntityDied,
            playerId: 3453,
            killerId: 65435,
            duration: 0,
            message: Some(ChatComponent::from("test".to_string())),
        })
    }

    #[test]
    fn test_cycle_enter() -> Result<()> {
        cycle(PlayerCombatPacket {
            event: PlayerCombatPacketEvent::EnterCombat,
            playerId: 0,
            killerId: 0,
            duration: 0,
            message: None,
        })
    }
}
