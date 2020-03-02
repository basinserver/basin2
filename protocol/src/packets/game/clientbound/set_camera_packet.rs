use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct SetCameraPacket {
    pub cameraId: i32,
}

impl CodablePacket for SetCameraPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.cameraId);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let cameraId = buf.get_mc_var_int()?;
        return Ok(SetCameraPacket { cameraId });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(SetCameraPacket { cameraId: 35667 })
    }
}
