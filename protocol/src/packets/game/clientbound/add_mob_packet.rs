use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;
use uuid::Uuid;

#[derive(PartialEq, Clone, Debug)]
pub struct AddMobPacket {
    pub id: i32,
    pub uuid: Uuid,
    pub entityType: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub xd: i16,
    pub yd: i16,
    pub zd: i16,
    pub yRot: u8,
    pub xRot: u8,
    pub yHeadRot: u8,
}

impl CodablePacket for AddMobPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.id);
        buf.set_mc_uuid(self.uuid);
        buf.set_mc_var_int(self.entityType);
        buf.set_mc_f64(self.x);
        buf.set_mc_f64(self.y);
        buf.set_mc_f64(self.z);
        buf.set_mc_u8(self.yRot);
        buf.set_mc_u8(self.xRot);
        buf.set_mc_u8(self.yHeadRot);
        buf.set_mc_i16(self.xd);
        buf.set_mc_i16(self.yd);
        buf.set_mc_i16(self.zd);
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
        let yRot = buf.get_mc_u8()?;
        let xRot = buf.get_mc_u8()?;
        let yHeadRot = buf.get_mc_u8()?;
        let xd = buf.get_mc_i16()?;
        let yd = buf.get_mc_i16()?;
        let zd = buf.get_mc_i16()?;
        return Ok(AddMobPacket {
            id,
            uuid,
            entityType,
            x,
            y,
            z,
            xd,
            yd,
            zd,
            yRot,
            xRot,
            yHeadRot,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(AddMobPacket {
            id : 54321,
            uuid: Uuid::new_v4(),
            entityType: 643,
            x: 123.0,
            y: 64.0,
            z: -157.0,
            xd: 10,
            yd: 20,
            zd: 30,
            xRot: 20,
            yHeadRot: 50,
            yRot: 30,
        })
    }
}