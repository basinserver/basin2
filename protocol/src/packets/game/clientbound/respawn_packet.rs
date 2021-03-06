use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct RespawnPacket {
    pub dimension: DimensionType,
    pub seed: i64,
    pub playerGameType: GameType,
    pub levelType: String,
}

impl CodablePacket for RespawnPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_i32(self.dimension.id());
        buf.set_mc_i64(self.seed);
        buf.set_mc_u8(self.playerGameType as u8);
        buf.set_mc_string(self.levelType);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let dimension =
            DimensionType::from_id(buf.get_mc_i32()?).unwrap_or(DimensionType::Overworld);
        let seed = buf.get_mc_i64()?;
        let playerGameType: GameType = buf.get_mc_enum()?;
        let levelType = buf.get_mc_string(16)?;
        return Ok(RespawnPacket {
            dimension,
            seed,
            playerGameType,
            levelType,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(RespawnPacket {
            dimension: DimensionType::Overworld,
            seed: 564645642,
            playerGameType: GameType::Survival,
            levelType: "default".to_string(),
        })
    }
}
