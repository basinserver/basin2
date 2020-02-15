use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

pub struct SetPassengersPacket {
    pub vehicle: i32,
    pub passengers: Vec<i32>,
}

impl CodablePacket for SetPassengersPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.vehicle);
        buf.set_mc_var_int_array(self.passengers);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let vehicle = buf.get_mc_var_int()?;
        let passengers = buf.get_mc_var_int_array()?;
        return Ok(SetPassengersPacket {
            vehicle,
            passengers,
        });
    }
}
