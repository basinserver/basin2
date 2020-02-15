use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;
use uuid::Uuid;

pub struct AddEntityPacket {
    pub id: i32,
    pub uuid: Uuid,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub xa: i16,
    pub ya: i16,
    pub za: i16,
    pub xRot: u8,
    pub yRot: u8,
    pub entityType: EntityType,
    pub data: i32,
}

impl CodablePacket for AddEntityPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.id);
        buf.set_mc_uuid(self.uuid);
        buf.set_mc_var_int(self.entityType);
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

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let id = buf.get_mc_var_int()?;
        let uuid = buf.get_mc_uuid()?;
        let entityType = buf.get_mc_var_int()?;
        let x = buf.get_mc_f64()?;
        let y = buf.get_mc_f64()?;
        let z = buf.get_mc_f64()?;
        let xRot = buf.get_mc_u8()?;
        let yRot = buf.get_mc_u8()?;
        let data = buf.get_mc_i32()?;
        let xa = buf.get_mc_i16()?;
        let ya = buf.get_mc_i16()?;
        let za = buf.get_mc_i16()?;
        return Ok(AddEntityPacket {
            id,
            uuid,
            x,
            y,
            z,
            xa,
            ya,
            za,
            xRot,
            yRot,
            entityType,
            data,
        });
    }
}
