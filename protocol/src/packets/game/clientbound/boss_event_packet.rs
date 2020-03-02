use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;
use uuid::Uuid;

#[derive(PartialEq, Clone, Debug)]
pub enum BossEventPacketData {
    Add(
        ChatComponent,
        f32,
        BossBarColor,
        BossBarOverlay,
        bool,
        bool,
        bool,
    ),
    Remove(),
    UpdatePct(f32),
    UpdateName(ChatComponent),
    UpdateStyle(BossBarColor, BossBarOverlay),
    UpdateProperties(bool, bool, bool),
}

#[derive(PartialEq, Clone, Debug)]
pub struct BossEventPacket {
    pub id: Uuid,
    pub operation: BossEventPacketOperation,
    pub data: BossEventPacketData,
}

impl CodablePacket for BossEventPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_uuid(self.id);
        buf.set_mc_var_int(self.operation as i32);
        match (self.operation, self.data) {
            (
                BossEventPacketOperation::Add,
                BossEventPacketData::Add(
                    name,
                    pct,
                    color,
                    overlay,
                    darkenScreen,
                    playMusic,
                    createWorldFog,
                ),
            ) => {
                buf.set_mc_chat_component(name);
                buf.set_mc_f32(pct);
                buf.set_mc_var_int(color as i32);
                buf.set_mc_var_int(overlay as i32);
                let mut flags = 0;
                if darkenScreen {
                    flags |= 1;
                }
                if playMusic {
                    flags |= 2;
                }
                if createWorldFog {
                    flags |= 4;
                }
                buf.set_mc_u8(flags);
            }
            (BossEventPacketOperation::Remove, BossEventPacketData::Remove()) => {}
            (BossEventPacketOperation::UpdatePct, BossEventPacketData::UpdatePct(pct)) => {
                buf.set_mc_f32(pct);
            }
            (BossEventPacketOperation::UpdateName, BossEventPacketData::UpdateName(name)) => {
                buf.set_mc_chat_component(name);
            }
            (
                BossEventPacketOperation::UpdateStyle,
                BossEventPacketData::UpdateStyle(color, overlay),
            ) => {
                buf.set_mc_var_int(color as i32);
                buf.set_mc_var_int(overlay as i32);
            }
            (
                BossEventPacketOperation::UpdateProperties,
                BossEventPacketData::UpdateProperties(darkenScreen, playMusic, createWorldFog),
            ) => {
                let mut flags = 0;
                if darkenScreen {
                    flags |= 1;
                }
                if playMusic {
                    flags |= 2;
                }
                if createWorldFog {
                    flags |= 4;
                }
                buf.set_mc_u8(flags);
            }
            _ => panic!("invalid formed outgoing boss_event_packet, mismatched types"),
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        use BossEventPacketOperation::*;

        let id = buf.get_mc_uuid()?;
        let operation: BossEventPacketOperation = buf.get_mc_enum()?;
        let data = match operation {
            Add => {
                let name = buf.get_mc_chat_component()?;
                let pct = buf.get_mc_f32()?;
                let color = buf.get_mc_enum::<BossBarColor>()?;
                let overlay = buf.get_mc_enum::<BossBarOverlay>()?;
                let flags = buf.get_mc_u8()?;
                BossEventPacketData::Add(
                    name,
                    pct,
                    color,
                    overlay,
                    (flags & 1) > 0,
                    (flags & 2) > 0,
                    (flags & 4) > 0,
                )
            }
            Remove => BossEventPacketData::Remove(),
            UpdatePct => BossEventPacketData::UpdatePct(buf.get_mc_f32()?),
            UpdateName => BossEventPacketData::UpdateName(buf.get_mc_chat_component()?),
            UpdateStyle => BossEventPacketData::UpdateStyle(
                buf.get_mc_enum::<BossBarColor>()?,
                buf.get_mc_enum::<BossBarOverlay>()?,
            ),
            UpdateProperties => {
                let flags = buf.get_mc_u8()?;
                BossEventPacketData::UpdateProperties(
                    (flags & 1) > 0,
                    (flags & 2) > 0,
                    (flags & 4) > 0,
                )
            }
        };
        return Ok(BossEventPacket {
            id,
            operation,
            data,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle_add() -> Result<()> {
        cycle(BossEventPacket {
            id: Uuid::new_v4(),
            operation: BossEventPacketOperation::Add,
            data: BossEventPacketData::Add(
                ChatComponent::from("test".to_string()),
                123.0,
                BossBarColor::Green,
                BossBarOverlay::Progress,
                true,
                false,
                true,
            ),
        })
    }

    #[test]
    fn test_cycle_remove() -> Result<()> {
        cycle(BossEventPacket {
            id: Uuid::new_v4(),
            operation: BossEventPacketOperation::Remove,
            data: BossEventPacketData::Remove(),
        })
    }

    #[test]
    fn test_cycle_update_pct() -> Result<()> {
        cycle(BossEventPacket {
            id: Uuid::new_v4(),
            operation: BossEventPacketOperation::UpdatePct,
            data: BossEventPacketData::UpdatePct(150.0),
        })
    }

    #[test]
    fn test_cycle_name() -> Result<()> {
        cycle(BossEventPacket {
            id: Uuid::new_v4(),
            operation: BossEventPacketOperation::UpdateName,
            data: BossEventPacketData::UpdateName(ChatComponent::from("new_name".to_string())),
        })
    }

    #[test]
    fn test_cycle_style() -> Result<()> {
        cycle(BossEventPacket {
            id: Uuid::new_v4(),
            operation: BossEventPacketOperation::UpdateStyle,
            data: BossEventPacketData::UpdateStyle(BossBarColor::Red, BossBarOverlay::Notched6),
        })
    }

    #[test]
    fn test_cycle_properties() -> Result<()> {
        cycle(BossEventPacket {
            id: Uuid::new_v4(),
            operation: BossEventPacketOperation::UpdateProperties,
            data: BossEventPacketData::UpdateProperties(true, false, true),
        })
    }
}
