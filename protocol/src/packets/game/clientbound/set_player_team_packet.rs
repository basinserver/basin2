use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct SetPlayerTeamPacket {
    pub name: String,
    pub displayName: Option<ChatComponent>,
    pub playerPrefix: Option<ChatComponent>,
    pub playerSuffix: Option<ChatComponent>,
    pub nametagVisibility: Option<String>,
    pub collisionRule: Option<String>,
    pub color: Option<ChatFormatting>,
    pub players: Option<Vec<String>>,
    pub method: i8,
    pub options: Option<i8>,
}

impl CodablePacket for SetPlayerTeamPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_string(self.name);
        buf.set_mc_i8(self.method);
        if self.method == 0 || self.method == 2 {
            buf.set_mc_chat_component(self.displayName.unwrap());
            buf.set_mc_i8(self.options.unwrap());
            buf.set_mc_string(self.nametagVisibility.unwrap());
            buf.set_mc_string(self.collisionRule.unwrap());
            buf.set_mc_var_int(self.color.unwrap() as i32);
            buf.set_mc_chat_component(self.playerPrefix.unwrap());
            buf.set_mc_chat_component(self.playerSuffix.unwrap());
        }

        if self.method == 0 || self.method == 3 || self.method == 4 {
            let players = self.players.unwrap();
            buf.set_mc_var_int(players.len() as i32);
            for player in players {
                buf.set_mc_string(player);
            }
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let name = buf.get_mc_string(16)?;
        let method = buf.get_mc_i8()?;
        let (
            displayName,
            options,
            nametagVisibility,
            collisionRule,
            color,
            playerPrefix,
            playerSuffix,
        ) = match method {
            0 | 2 => (
                Some(buf.get_mc_chat_component()?),
                Some(buf.get_mc_i8()?),
                Some(buf.get_mc_string(40)?),
                Some(buf.get_mc_string(40)?),
                Some(buf.get_mc_enum::<ChatFormatting>()?),
                Some(buf.get_mc_chat_component()?),
                Some(buf.get_mc_chat_component()?),
            ),
            _ => (None, None, None, None, None, None, None),
        };
        let players = match method {
            0 | 3 | 4 => {
                let mut players: Vec<String> = vec![];
                let count = buf.get_mc_var_int()?;
                for _ in 0..count {
                    players.push(buf.get_mc_string(40)?);
                }
                Some(players)
            }
            _ => None,
        };
        return Ok(SetPlayerTeamPacket {
            name,
            displayName,
            playerPrefix,
            playerSuffix,
            nametagVisibility,
            collisionRule,
            color,
            players,
            method,
            options,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle_method0() -> Result<()> {
        cycle(SetPlayerTeamPacket {
            name: "team name".to_string(),
            displayName: Some(ChatComponent::from("display".to_string())),
            playerPrefix: Some(ChatComponent::from("prefix".to_string())),
            playerSuffix: Some(ChatComponent::from("suffix".to_string())),
            nametagVisibility: Some("nametag".to_string()),
            collisionRule: Some("collision".to_string()),
            color: Some(ChatFormatting::Green),
            players: Some(vec!["player1".to_string(), "player2".to_string()]),
            method: 0,
            options: Some(12),
        })
    }

    #[test]
    fn test_cycle_method2() -> Result<()> {
        cycle(SetPlayerTeamPacket {
            name: "team name".to_string(),
            displayName: Some(ChatComponent::from("display".to_string())),
            playerPrefix: Some(ChatComponent::from("prefix".to_string())),
            playerSuffix: Some(ChatComponent::from("suffix".to_string())),
            nametagVisibility: Some("nametag".to_string()),
            collisionRule: Some("collision".to_string()),
            color: Some(ChatFormatting::Green),
            players: None,
            method: 2,
            options: Some(12),
        })
    }

    #[test]
    fn test_cycle_method3() -> Result<()> {
        cycle(SetPlayerTeamPacket {
            name: "team name".to_string(),
            displayName: None,
            playerPrefix: None,
            playerSuffix: None,
            nametagVisibility: None,
            collisionRule: None,
            color: None,
            players: Some(vec!["player1".to_string(), "player2".to_string()]),
            method: 3,
            options: None,
        })
    }
}
