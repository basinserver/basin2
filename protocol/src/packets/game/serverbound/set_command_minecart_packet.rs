use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub struct SetCommandMinecartPacket {
    pub entity: i32,
    pub command: String,
    pub trackOutput: bool,
}

impl CodablePacket for SetCommandMinecartPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.entity);
        buf.set_mc_string(self.command);
        buf.set_mc_bool(self.trackOutput);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let entity = buf.get_mc_var_int()?;
        let command = buf.get_mc_string(32767)?;
        let trackOutput = buf.get_mc_bool()?;
        return Ok(SetCommandMinecartPacket {
            entity,
            command,
            trackOutput,
        });
    }
}
