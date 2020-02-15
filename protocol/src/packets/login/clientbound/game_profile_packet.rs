
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct GameProfilePacket {
    pub gameProfile: GameProfile,
}

impl CodablePacket for GameProfilePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_string(self.gameProfile.uuid.map(|uuid| uuid.hyphenated().to_string()).unwrap_or("".to_string()));
        buf.set_mc_string(self.gameProfile.name);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let gameProfile = GameProfile {
            legacy: false,
            uuid: Some(Uuid::parse_str(&*buf.get_mc_string(36)?)?),
            name: buf.get_mc_string(16)?,
        };
        return Ok(GameProfilePacket { gameProfile });
    }
}
