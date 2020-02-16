use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct PlayerPositionPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yRot: f32,
    pub xRot: f32,
    pub relativeArguments: (bool, bool, bool, bool, bool),
    pub id: i32,
}

impl CodablePacket for PlayerPositionPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_f64(self.x);
        buf.set_mc_f64(self.y);
        buf.set_mc_f64(self.z);
        buf.set_mc_f32(self.yRot);
        buf.set_mc_f32(self.xRot);
        let mut flags = 0;
        if self.relativeArguments.0 {
            flags |= 1;
        }
        if self.relativeArguments.1 {
            flags |= 2;
        }
        if self.relativeArguments.2 {
            flags |= 4;
        }
        if self.relativeArguments.3 {
            flags |= 8;
        }
        if self.relativeArguments.4 {
            flags |= 16;
        }
        buf.set_mc_u8(flags);
        buf.set_mc_var_int(self.id);
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
        let flags = buf.get_mc_u8()?;
        let relativeArguments = (
            (flags & 1) > 0,
            (flags & 2) > 0,
            (flags & 4) > 0,
            (flags & 8) > 0,
            (flags & 16) > 0,
        );
        let id = buf.get_mc_var_int()?;
        return Ok(PlayerPositionPacket {
            x,
            y,
            z,
            yRot,
            xRot,
            relativeArguments,
            id,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(PlayerPositionPacket {
            x: -45.0,
            y: 64.5,
            z: 265.34,
            yRot: 120.5,
            xRot: 19.345,
            relativeArguments: (false, true, false, true, true),
            id: 34534,
        })
    }
}
