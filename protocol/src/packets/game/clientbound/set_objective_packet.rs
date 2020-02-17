use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct SetObjectivePacket {
    pub objectiveName: String,
    pub displayName: Option<ChatComponent>,
    pub renderType: Option<ObjectiveCriteriaRenderType>,
    pub method: i8,
}

impl CodablePacket for SetObjectivePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_string(self.objectiveName);
        buf.set_mc_i8(self.method);
        if self.method == 0 || self.method == 2 {
            buf.set_mc_chat_component(self.displayName.unwrap());
            buf.set_mc_var_int(self.renderType.unwrap() as i32);
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let objectiveName = buf.get_mc_string(16)?;
        let method = buf.get_mc_i8()?;
        let (displayName, renderType) = match method {
            0 | 2 => (
                Some(buf.get_mc_chat_component()?),
                Some(buf.get_mc_enum::<ObjectiveCriteriaRenderType>()?),
            ),
            _ => (None, None),
        };
        return Ok(SetObjectivePacket {
            objectiveName,
            displayName,
            renderType,
            method,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle_lowmethod() -> Result<()> {
        cycle(SetObjectivePacket {
            objectiveName: "objective name".to_string(),
            displayName: Some(ChatComponent::from("display objective".to_string())),
            renderType: Some(ObjectiveCriteriaRenderType::Hearts),
            method: 2,
        })
    }

    #[test]
    fn test_cycle_highmethod() -> Result<()> {
        cycle(SetObjectivePacket {
            objectiveName: "objective name".to_string(),
            displayName: None,
            renderType: None,
            method: 4,
        })
    }
}
