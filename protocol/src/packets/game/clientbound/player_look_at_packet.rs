
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;

pub struct PlayerLookAtPacket {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub fromAnchor: EntityAnchor,
    pub atEntity: Option<(i32, EntityAnchor)>,
}

impl CodablePacket for PlayerLookAtPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.fromAnchor as i32);
        buf.set_mc_f64(self.x);
        buf.set_mc_f64(self.y);
        buf.set_mc_f64(self.z);
        match self.atEntity {
            Some((entity, toAnchor)) => {
                buf.set_mc_bool(true);
                buf.set_mc_var_int(entity);
                buf.set_mc_var_int(toAnchor as i32);    
            },
            None => {
                buf.set_mc_bool(false);
            }
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let fromAnchor: EntityAnchor = buf.get_mc_enum()?;
        let x = buf.get_mc_f64()?;
        let y = buf.get_mc_f64()?;
        let z = buf.get_mc_f64()?;
        let atEntity = if buf.get_mc_bool()? {
            Some((
                buf.get_mc_var_int()?,
                buf.get_mc_enum()?,
            ))
        } else {
            None
        };
        return Ok(PlayerLookAtPacket { x, y, z, fromAnchor, atEntity });
    }
}
