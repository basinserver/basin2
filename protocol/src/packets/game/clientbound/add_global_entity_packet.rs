use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct AddGlobalEntityPacket {
    pub id: i32,
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub entityType: u8,
}

impl CodablePacket for AddGlobalEntityPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.id);
        buf.set_mc_u8(self.entityType);
        buf.set_mc_f64(self.x);
        buf.set_mc_f64(self.y);
        buf.set_mc_f64(self.z);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let id = buf.get_mc_var_int()?;
        let entityType = buf.get_mc_u8()?;
        let x = buf.get_mc_f64()?;
        let y = buf.get_mc_f64()?;
        let z = buf.get_mc_f64()?;
        return Ok(AddGlobalEntityPacket {
            id,
            x,
            y,
            z,
            entityType,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(AddGlobalEntityPacket {
            id: 54321,
            x: 123.0,
            y: 64.0,
            z: -157.0,
            entityType: 47,
        })
    }
}
