use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct LightUpdatePacket {
    pub x: i32,
    pub z: i32,
    pub skyYMask: i32,
    pub blockYMask: i32,
    pub emptySkyYMask: i32,
    pub emptyBlockYMask: i32,
    pub skyUpdates: Vec<Vec<u8>>,
    pub blockUpdates: Vec<Vec<u8>>,
}

impl CodablePacket for LightUpdatePacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.x);
        buf.set_mc_var_int(self.z);
        buf.set_mc_var_int(self.skyYMask);
        buf.set_mc_var_int(self.blockYMask);
        buf.set_mc_var_int(self.emptySkyYMask);
        buf.set_mc_var_int(self.emptyBlockYMask);
        for skyUpdate in self.skyUpdates {
            buf.set_mc_byte_array(skyUpdate);
        }
        for blockUpdate in self.blockUpdates {
            buf.set_mc_byte_array(blockUpdate);
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let x = buf.get_mc_var_int()?;
        let z = buf.get_mc_var_int()?;
        let skyYMask = buf.get_mc_var_int()?;
        let blockYMask = buf.get_mc_var_int()?;
        let emptySkyYMask = buf.get_mc_var_int()?;
        let emptyBlockYMask = buf.get_mc_var_int()?;
        let mut skyUpdates: Vec<Vec<u8>> = vec![];
        for i in 0..18 {
            if (skyYMask & (1 << i)) != 0 {
                skyUpdates.push(buf.get_mc_byte_array_bounded(2048)?);
            }
        }
        let mut blockUpdates: Vec<Vec<u8>> = vec![];
        for i in 0..18 {
            if (blockYMask & (1 << i)) != 0 {
                blockUpdates.push(buf.get_mc_byte_array_bounded(2048)?);
            }
        }
        return Ok(LightUpdatePacket {
            x,
            z,
            skyYMask,
            blockYMask,
            emptySkyYMask,
            emptyBlockYMask,
            skyUpdates,
            blockUpdates,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(LightUpdatePacket {
            x: 12,
            z: 432,
            skyYMask: 1,
            blockYMask: 2,
            emptyBlockYMask: 4,
            emptySkyYMask: 8,
            skyUpdates: vec![vec![12].repeat(2048)],
            blockUpdates: vec![vec![34].repeat(2048)],
        })
    }
}