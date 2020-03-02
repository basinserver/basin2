use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct MovePlayerRotPacket {
    pub yRot: f32,
    pub xRot: f32,
    pub onGround: bool,
}

impl CodablePacket for MovePlayerRotPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_f32(self.yRot);
        buf.set_mc_f32(self.xRot);
        buf.set_mc_bool(self.onGround);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let yRot = buf.get_mc_f32()?;
        let xRot = buf.get_mc_f32()?;
        let onGround = buf.get_mc_bool()?;
        return Ok(MovePlayerRotPacket {
            yRot,
            xRot,
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
        cycle(MovePlayerRotPacket {
            yRot: 120.0,
            xRot: 130.0,
            onGround: false,
        })
    }
}
