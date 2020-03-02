use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct MoveVehiclePacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yRot: f32,
    pub xRot: f32,
}

impl CodablePacket for MoveVehiclePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_f64(self.x);
        buf.set_mc_f64(self.y);
        buf.set_mc_f64(self.z);
        buf.set_mc_f32(self.yRot);
        buf.set_mc_f32(self.xRot);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let x = buf.get_mc_f64()?;
        let y = buf.get_mc_f64()?;
        let z = buf.get_mc_f64()?;
        let yRot = buf.get_mc_f32()?;
        let xRot = buf.get_mc_f32()?;
        return Ok(MoveVehiclePacket {
            x,
            y,
            z,
            yRot,
            xRot,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(MoveVehiclePacket {
            x: 1.0,
            y: 2.0,
            z: 3.0,
            yRot: 120.0,
            xRot: 130.0,
        })
    }
}
