use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;
use uuid::Uuid;

#[derive(PartialEq, Clone, Debug)]
pub struct GameProfilePacket {
    pub gameProfile: GameProfile,
}

impl CodablePacket for GameProfilePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_string(
            self.gameProfile
                .uuid
                .map(|uuid| uuid.hyphenated().to_string())
                .unwrap_or("".to_string()),
        );
        buf.set_mc_string(self.gameProfile.name);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let gameProfile = GameProfile {
            legacy: false,
            uuid: Some(Uuid::parse_str(&*buf.get_mc_string(36)?)?),
            name: buf.get_mc_string(16)?,
        };
        return Ok(GameProfilePacket { gameProfile });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(GameProfilePacket {
            gameProfile: GameProfile {
                uuid: Some(Uuid::new_v4()),
                name: "testUser".to_string(),
                legacy: false,
            },
        })
    }
}
