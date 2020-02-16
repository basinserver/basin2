use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct RemoveEntitiesPacket {
    pub entityIds: Vec<i32>,
}

impl CodablePacket for RemoveEntitiesPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.entityIds.len() as i32);
        for entityId in self.entityIds {
            buf.set_mc_var_int(entityId as i32);
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let mut entityIds: Vec<i32> = vec![];
        let count = buf.get_mc_var_int()?;
        for _ in 0..count {
            entityIds.push(buf.get_mc_var_int()?);
        }
        return Ok(RemoveEntitiesPacket { entityIds });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(RemoveEntitiesPacket {
            entityIds: vec![345, 3454, 45674674],
        })
    }
}
