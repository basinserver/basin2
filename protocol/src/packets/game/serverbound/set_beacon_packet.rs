use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(SetBeaconPacket {
            primary: 7,
            secondary: 10,
        })
    }
}
