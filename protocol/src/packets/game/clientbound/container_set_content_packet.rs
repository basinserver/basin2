use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct ContainerSetContentPacket {
    pub containerId: u8,
    pub items: Vec<ItemStack>,
}

impl CodablePacket for ContainerSetContentPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_u8(self.containerId);
        buf.set_mc_u16(self.items.len() as u16);
        for item in self.items {
            buf.set_mc_item_stack(item);
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let containerId = buf.get_mc_u8()?;
        let mut items: Vec<ItemStack> = vec![];
        let item_count = buf.get_mc_u16()?;
        for _ in 0..item_count {
            items.push(buf.get_mc_item_stack()?);
        }
        return Ok(ContainerSetContentPacket { containerId, items });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(ContainerSetContentPacket {
            containerId: 12,
            items: vec![ItemStack::empty(), ItemStack::empty()],
        })
    }
}