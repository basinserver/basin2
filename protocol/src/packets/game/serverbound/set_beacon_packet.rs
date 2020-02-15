use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub struct SetBeaconPacket {
    pub primary: i32,
    pub secondary: i32,
}

impl CodablePacket for SetBeaconPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.primary);
        buf.set_mc_var_int(self.secondary);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let primary = buf.get_mc_var_int()?;
        let secondary = buf.get_mc_var_int()?;
        return Ok(SetBeaconPacket { primary, secondary });
    }
}
