use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct MoveEntityRotPacket {
    pub entityId: i32,
    pub yRot: i8,
    pub xRot: i8,
    pub onGround: bool,
}

impl CodablePacket for MoveEntityRotPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.entityId);
        buf.set_mc_i8(self.yRot);
        buf.set_mc_i8(self.xRot);
        buf.set_mc_bool(self.onGround);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let entityId = buf.get_mc_var_int()?;
        let yRot = buf.get_mc_i8()?;
        let xRot = buf.get_mc_i8()?;
        let onGround = buf.get_mc_bool()?;
        return Ok(MoveEntityRotPacket {
            entityId,
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
        cycle(MoveEntityRotPacket {
            entityId: 234,
            yRot: 12,
            xRot: -56,
            onGround: true,
        })
    }
}
