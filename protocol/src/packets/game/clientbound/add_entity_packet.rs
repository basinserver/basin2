
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

pub struct AddEntityPacket {
    pub id: i32,
    pub uuid: Uuid,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub xa: i32,
    pub ya: i32,
    pub za: i32,
    pub xRot: i32,
    pub yRot: i32,
    pub type: EntityType,
    pub data: i32,
}

impl CodablePacket for AddEntityPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.id);
        buf.set_mc_uuid(self.uuid);
        // TODO: UNKNOWN: var1.writeVarInt(Registry.ENTITY_TYPE.getId(this.type));
        buf.set_mc_f64(self.x);
        buf.set_mc_f64(self.y);
        buf.set_mc_f64(self.z);
        buf.set_mc_u8(self.xRot);
        buf.set_mc_u8(self.yRot);
        buf.set_mc_i32(self.data);
        buf.set_mc_i16(self.xa);
        buf.set_mc_i16(self.ya);
        buf.set_mc_i16(self.za);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let id = buf.get_mc_var_int()?;
        let uuid = buf.get_mc_uuid()?;
        // TODO: UNKNOWN: this.type = (EntityType)Registry.ENTITY_TYPE.byId(var1.readVarInt());
        let x = buf.get_mc_f64()?;
        let y = buf.get_mc_f64()?;
        let z = buf.get_mc_f64()?;
        let xRot = buf.get_mc_u8()?;
        let yRot = buf.get_mc_u8()?;
        let data = buf.get_mc_i32()?;
        let xa = buf.get_mc_i16()?;
        let ya = buf.get_mc_i16()?;
        let za = buf.get_mc_i16()?;
        return Ok(AddEntityPacket { id, uuid, x, y, z, xa, ya, za, xRot, yRot, type, data });
    }
}
