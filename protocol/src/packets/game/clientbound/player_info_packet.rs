use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;
use uuid::Uuid;

pub enum PlayerInfoPacketData {
    AddPlayer(String, Vec<PlayerProperty>, GameType, i32, Option<String>),
    UpdateGameMode(GameType),
    UpdateLatency(i32),
    UpdateDisplayName(Option<String>),
    RemovePlayer(),
}

pub struct PlayerInfoPacket {
    pub action: PlayerInfoPacketAction,
    pub data: Vec<(Uuid, PlayerInfoPacketData)>,
}

impl CodablePacket for PlayerInfoPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.action as i32);
        buf.set_mc_var_int(self.data.len() as i32);
        for (uuid, datum) in self.data {
            buf.set_mc_uuid(uuid);
            match (self.action, datum) {
                (
                    PlayerInfoPacketAction::AddPlayer,
                    PlayerInfoPacketData::AddPlayer(
                        name,
                        properties,
                        game_type,
                        latency,
                        display_name,
                    ),
                ) => {
                    buf.set_mc_string(name);
                    buf.set_mc_var_int(properties.len() as i32);
                    for property in properties {
                        buf.set_mc_string(property.name);
                        buf.set_mc_string(property.value);
                        match property.signature {
                            Some(signature) => {
                                buf.set_mc_bool(true);
                                buf.set_mc_string(signature);
                            }
                            None => {
                                buf.set_mc_bool(false);
                            }
                        }
                    }
                    buf.set_mc_var_int(game_type as i32);
                    buf.set_mc_var_int(latency);
                    match display_name {
                        Some(display_name) => {
                            buf.set_mc_bool(true);
                            buf.set_mc_string(display_name);
                        }
                        None => {
                            buf.set_mc_bool(false);
                        }
                    }
                }
                (
                    PlayerInfoPacketAction::UpdateGameMode,
                    PlayerInfoPacketData::UpdateGameMode(game_type),
                ) => {
                    buf.set_mc_var_int(game_type as i32);
                }
                (
                    PlayerInfoPacketAction::UpdateLatency,
                    PlayerInfoPacketData::UpdateLatency(latency),
                ) => {
                    buf.set_mc_var_int(latency);
                }
                (
                    PlayerInfoPacketAction::UpdateDisplayName,
                    PlayerInfoPacketData::UpdateDisplayName(Some(display_name)),
                ) => {
                    buf.set_mc_bool(true);
                    buf.set_mc_string(display_name);
                }
                (
                    PlayerInfoPacketAction::UpdateDisplayName,
                    PlayerInfoPacketData::UpdateDisplayName(None),
                ) => {
                    buf.set_mc_bool(false);
                }
                (PlayerInfoPacketAction::RemovePlayer, PlayerInfoPacketData::RemovePlayer()) => {}
                _ => panic!("invalid formed outgoing player_info_packet, mismatched types"),
            }
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        use PlayerInfoPacketAction::*;

        let action: PlayerInfoPacketAction = buf.get_mc_enum()?;
        let data_count = buf.get_mc_var_int()?;
        let mut data: Vec<(Uuid, PlayerInfoPacketData)> = vec![];
        for _ in 0..data_count {
            let uuid = buf.get_mc_uuid()?;
            let datum = match action {
                AddPlayer => {
                    let name = buf.get_mc_string(16)?;
                    let property_count = buf.get_mc_var_int()?;
                    let mut properties: Vec<PlayerProperty> = vec![];
                    for _ in 0..property_count {
                        let name = buf.get_mc_string(32767)?;
                        let value = buf.get_mc_string(32767)?;
                        let signature = if buf.get_mc_bool()? {
                            Some(buf.get_mc_string(32767)?)
                        } else {
                            None
                        };
                        properties.push(PlayerProperty {
                            name,
                            value,
                            signature,
                        });
                    }
                    let game_type: GameType = buf.get_mc_enum()?;
                    let latency = buf.get_mc_var_int()?;
                    let display_name = if buf.get_mc_bool()? {
                        Some(buf.get_mc_chat_component()?)
                    } else {
                        None
                    };
                    PlayerInfoPacketData::AddPlayer(
                        name,
                        properties,
                        game_type,
                        latency,
                        display_name,
                    )
                }
                UpdateGameMode => {
                    let game_type: GameType = buf.get_mc_enum()?;
                    PlayerInfoPacketData::UpdateGameMode(game_type)
                }
                UpdateLatency => {
                    let latency = buf.get_mc_var_int()?;
                    PlayerInfoPacketData::UpdateLatency(latency)
                }
                UpdateDisplayName => {
                    let display_name = if buf.get_mc_bool()? {
                        Some(buf.get_mc_chat_component()?)
                    } else {
                        None
                    };

                    PlayerInfoPacketData::UpdateDisplayName(display_name)
                }
                RemovePlayer => PlayerInfoPacketData::RemovePlayer(),
            };
            data.push((uuid, datum));
        }
        return Ok(PlayerInfoPacket { action, data });
    }
}
