use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;
use enum_primitive::FromPrimitive;

pub struct LoginPacket {
    pub playerId: i32,
    pub seed: i64,
    pub hardcore: bool,
    pub gameType: GameType,
    pub dimension: DimensionType,
    pub maxPlayers: u8,
    pub levelType: String,
    pub chunkRadius: i32,
    pub reducedDebugInfo: bool,
    pub showDeathScreen: bool,
}

impl CodablePacket for LoginPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_i32(self.playerId);
        let mut flags = self.gameType as u8;
        if self.hardcore {
            flags |= 8;
        }
        buf.set_mc_u8(flags);
        buf.set_mc_i32(self.dimension.id());
        buf.set_mc_i64(self.seed);
        buf.set_mc_u8(self.maxPlayers);
        buf.set_mc_string(self.levelType);
        buf.set_mc_var_int(self.chunkRadius);
        buf.set_mc_bool(self.reducedDebugInfo);
        buf.set_mc_bool(self.showDeathScreen);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let playerId = buf.get_mc_i32()?;
        let flags = buf.get_mc_u8()?;
        let hardcore = (flags & 8) != 0;
        let gameType = GameType::from_i32((flags & 0b111) as i32).unwrap_or(GameType::Survival);
        let dimension =
            DimensionType::from_id(buf.get_mc_i32()?).unwrap_or(DimensionType::Overworld);
        let seed = buf.get_mc_i64()?;
        let maxPlayers = buf.get_mc_u8()?;
        let levelType = buf.get_mc_string(16)?;
        let chunkRadius = buf.get_mc_var_int()?;
        let reducedDebugInfo = buf.get_mc_bool()?;
        let showDeathScreen = buf.get_mc_bool()?;
        return Ok(LoginPacket {
            playerId,
            seed,
            hardcore,
            gameType,
            dimension,
            maxPlayers,
            levelType,
            chunkRadius,
            reducedDebugInfo,
            showDeathScreen,
        });
    }
}
