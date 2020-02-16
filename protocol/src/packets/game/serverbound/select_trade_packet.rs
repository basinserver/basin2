use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct SelectTradePacket {
    pub item: i32,
}

impl CodablePacket for SelectTradePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.item);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let item = buf.get_mc_var_int()?;
        return Ok(SelectTradePacket { item });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(SelectTradePacket {
            item: 7,
        })
    }
}