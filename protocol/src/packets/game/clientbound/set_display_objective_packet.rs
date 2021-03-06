use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct SetDisplayObjectivePacket {
    pub slot: u8,
    pub objectiveName: String,
}

impl CodablePacket for SetDisplayObjectivePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_u8(self.slot);
        buf.set_mc_string(self.objectiveName);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let slot = buf.get_mc_u8()?;
        let objectiveName = buf.get_mc_string(16)?;
        return Ok(SetDisplayObjectivePacket {
            slot,
            objectiveName,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(SetDisplayObjectivePacket {
            slot: 12,
            objectiveName: "an objective".to_string(),
        })
    }
}
