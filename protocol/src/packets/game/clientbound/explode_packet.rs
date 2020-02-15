
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct ExplodePacket {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub power: f32,
    pub toBlow: Vec<BlockPos>,
    pub knockbackX: f32,
    pub knockbackY: f32,
    pub knockbackZ: f32,
}

impl CodablePacket for ExplodePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_f32(self.x);
        buf.set_mc_f32(self.y);
        buf.set_mc_f32(self.z);
        buf.set_mc_f32(self.power);
        buf.set_mc_i32(self.toBlow.len() as i32);
        let int_x = self.x.floor() as i32;
        let int_y = self.y.floor() as i32;
        let int_z = self.z.floor() as i32;
        for pos in self.toBlow {
            buf.set_mc_i8((pos.x - int_x) as i8);
            buf.set_mc_i8((pos.y - int_y) as i8);
            buf.set_mc_i8((pos.z - int_z) as i8);
        }
        buf.set_mc_f32(self.knockbackX);
        buf.set_mc_f32(self.knockbackY);
        buf.set_mc_f32(self.knockbackZ);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let x = buf.get_mc_f32()?;
        let y = buf.get_mc_f32()?;
        let z = buf.get_mc_f32()?;
        let power = buf.get_mc_f32()?;
        let item_count = buf.get_mc_i32()?;
        let mut toBlow: Vec<BlockPos> = vec![];
        let int_x = x.floor() as i32;
        let int_y = y.floor() as i32;
        let int_z = z.floor() as i32;
        for _ in 0..item_count {
            toBlow.push(BlockPos {
                x: (buf.get_mc_i8()? as i32 + int_x),
                y: (buf.get_mc_i8()? as i32 + int_y),
                z: (buf.get_mc_i8()? as i32 + int_z),
            })
        }
        let knockbackX = buf.get_mc_f32()?;
        let knockbackY = buf.get_mc_f32()?;
        let knockbackZ = buf.get_mc_f32()?;
        return Ok(ExplodePacket { x, y, z, power, toBlow, knockbackX, knockbackY, knockbackZ });
    }
}
