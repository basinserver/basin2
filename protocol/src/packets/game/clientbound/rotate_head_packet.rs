use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct RotateHeadPacket {
    pub entityId: i32,
    pub yHeadRot: u8,
}

impl CodablePacket for RotateHeadPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.entityId);
        buf.set_mc_u8(self.yHeadRot);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let entityId = buf.get_mc_var_int()?;
        let yHeadRot = buf.get_mc_u8()?;
        return Ok(RotateHeadPacket { entityId, yHeadRot });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(RotateHeadPacket {
            entityId: 45369,
            yHeadRot: 23,
        })
    }
}
