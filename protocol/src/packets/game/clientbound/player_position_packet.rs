
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct PlayerPositionPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub yRot: f32,
    pub xRot: f32,
    pub relativeArguments: undefined,
    pub id: i32,
}

impl CodablePacket for PlayerPositionPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_f64(self.x);
        buf.set_mc_f64(self.y);
        buf.set_mc_f64(self.z);
        buf.set_mc_f32(self.yRot);
        buf.set_mc_f32(self.xRot);
        // TODO: UNKNOWN: var1.writeByte(ClientboundPlayerPositionPacket.RelativeArgument.pack(this.relativeArguments));
        buf.set_mc_var_int(self.id);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let x = buf.get_mc_f64()?;
        let y = buf.get_mc_f64()?;
        let z = buf.get_mc_f64()?;
        let yRot = buf.get_mc_f32()?;
        let xRot = buf.get_mc_f32()?;
        // TODO: UNKNOWN: this.relativeArguments = ClientboundPlayerPositionPacket.RelativeArgument.unpack(var1.readUnsignedByte());
        let id = buf.get_mc_var_int()?;
        return Ok(PlayerPositionPacket { x, y, z, yRot, xRot, relativeArguments, id });
    }
}
