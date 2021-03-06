use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct MoveEntityPosPacket {
    pub entityId: i32,
    pub xa: i16,
    pub ya: i16,
    pub za: i16,
    pub onGround: bool,
}

impl CodablePacket for MoveEntityPosPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.entityId);
        buf.set_mc_i16(self.xa);
        buf.set_mc_i16(self.ya);
        buf.set_mc_i16(self.za);
        buf.set_mc_bool(self.onGround);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let entityId = buf.get_mc_var_int()?;
        let xa = buf.get_mc_i16()?;
        let ya = buf.get_mc_i16()?;
        let za = buf.get_mc_i16()?;
        let onGround = buf.get_mc_bool()?;
        return Ok(MoveEntityPosPacket {
            entityId,
            xa,
            ya,
            za,
            onGround,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(MoveEntityPosPacket {
            entityId: 234,
            xa: 12,
            ya: 56,
            za: -453,
            onGround: true,
        })
    }
}
