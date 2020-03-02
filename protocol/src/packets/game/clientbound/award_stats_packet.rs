use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct AwardStatsPacketItem {
    stat_type: i32,
    stat: i32,
    value: i32,
}

#[derive(PartialEq, Clone, Debug)]
pub struct AwardStatsPacket {
    pub stats: Vec<AwardStatsPacketItem>,
}

impl CodablePacket for AwardStatsPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.stats.len() as i32);
        for item in self.stats {
            buf.set_mc_var_int(item.stat_type);
            buf.set_mc_var_int(item.stat);
            buf.set_mc_var_int(item.value);
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let mut stats: Vec<AwardStatsPacketItem> = vec![];
        let count = buf.get_mc_var_int()?;
        for _ in 0..count {
            stats.push(AwardStatsPacketItem {
                stat_type: buf.get_mc_var_int()?,
                stat: buf.get_mc_var_int()?,
                value: buf.get_mc_var_int()?,
            })
        }
        return Ok(AwardStatsPacket { stats });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(AwardStatsPacket {
            stats: vec![AwardStatsPacketItem {
                stat_type: 12,
                stat: 20,
                value: 100,
            }],
        })
    }
}
