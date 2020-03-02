use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct SetHealthPacket {
    pub health: f32,
    pub food: i32,
    pub saturation: f32,
}

impl CodablePacket for SetHealthPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_f32(self.health);
        buf.set_mc_var_int(self.food);
        buf.set_mc_f32(self.saturation);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let health = buf.get_mc_f32()?;
        let food = buf.get_mc_var_int()?;
        let saturation = buf.get_mc_f32()?;
        return Ok(SetHealthPacket {
            health,
            food,
            saturation,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(SetHealthPacket {
            health: 120.0,
            food: 20,
            saturation: 19.0,
        })
    }
}
