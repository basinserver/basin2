use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct SetPassengersPacket {
    pub vehicle: i32,
    pub passengers: Vec<i32>,
}

impl CodablePacket for SetPassengersPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.vehicle);
        buf.set_mc_var_int(self.passengers.len() as i32);
        for passenger in self.passengers {
            buf.set_mc_var_int(passenger);
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let vehicle = buf.get_mc_var_int()?;
        let mut passengers: Vec<i32> = vec![];
        let passenger_count = buf.get_mc_var_int()?;
        for _ in 0..passenger_count {
            passengers.push(buf.get_mc_var_int()?);
        }
        return Ok(SetPassengersPacket {
            vehicle,
            passengers,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(SetPassengersPacket {
            vehicle: 34598,
            passengers: vec![465343, 453, 67543],
        })
    }
}