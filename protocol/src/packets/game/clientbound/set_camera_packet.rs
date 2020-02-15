
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct SetCameraPacket {
    pub cameraId: i32,
}

impl CodablePacket for SetCameraPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.cameraId);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let cameraId = buf.get_mc_var_int()?;
        return Ok(SetCameraPacket { cameraId });
    }
}
