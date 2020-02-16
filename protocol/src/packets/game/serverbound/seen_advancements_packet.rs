use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct SeenAdvancementsPacket {
    pub action: SeenAdvancementsPacketAction,
    pub tab: Option<ResourceLocation>,
}

impl CodablePacket for SeenAdvancementsPacket {
    fn encode(self, buf: &mut BytesMut) {
        use SeenAdvancementsPacketAction::*;
        buf.set_mc_var_int(self.action as i32);
        match self.action {
            OpenedTab => {
                buf.set_mc_string(self.tab.unwrap());
            }
            _ => (),
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        use SeenAdvancementsPacketAction::*;

        let action: SeenAdvancementsPacketAction = buf.get_mc_enum()?;
        let tab = match action {
            OpenedTab => Some(buf.get_mc_string(32767)?),
            _ => None,
        };
        return Ok(SeenAdvancementsPacket { action, tab });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(SeenAdvancementsPacket {
            action: SeenAdvancementsPacketAction::OpenedTab,
            tab: Some("the_string".to_string()),
        })
    }
}
