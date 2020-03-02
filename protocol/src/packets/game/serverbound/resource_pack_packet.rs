use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct ResourcePackPacket {
    pub action: ResourcePackPacketAction,
}

impl CodablePacket for ResourcePackPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.action as i32);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let action: ResourcePackPacketAction = buf.get_mc_enum()?;
        return Ok(ResourcePackPacket { action });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(ResourcePackPacket {
            action: ResourcePackPacketAction::SuccessfullyLoaded,
        })
    }
}
