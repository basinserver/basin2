use basin2_lib::nbt::*;
use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct LevelChunkPacket {
    pub x: i32,
    pub z: i32,
    pub availableSections: i32,
    pub heightmaps: Nbt,
    pub biomes: Option<Box<Vec<i32>>>,
    pub buffer: BytesMut,
    pub blockEntitiesTags: Vec<Nbt>,
}

impl CodablePacket for LevelChunkPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_i32(self.x);
        buf.set_mc_i32(self.z);
        buf.set_mc_bool(self.biomes.is_some());
        buf.set_mc_var_int(self.availableSections);
        buf.set_mc_nbt(self.heightmaps);
        match self.biomes {
            Some(biomes) => {
                for biome in biomes.iter() {
                    buf.set_mc_i32(*biome);
                }
            }
            None => (),
        }
        buf.set_mc_var_int(self.buffer.len() as i32);
        buf.unsplit(self.buffer);
        buf.set_mc_var_int(self.blockEntitiesTags.len() as i32);
        for blockEntitiesTag in self.blockEntitiesTags {
            buf.set_mc_nbt(blockEntitiesTag);
        }
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let x = buf.get_mc_i32()?;
        let z = buf.get_mc_i32()?;
        let has_biomes = buf.get_mc_bool()?;
        let availableSections = buf.get_mc_var_int()?;
        let heightmaps = buf.get_mc_nbt()?;
        let biomes = if has_biomes {
            let mut biomes = Box::new(vec![]);
            for _ in 0..(1 << 10) {
                biomes.push(buf.get_mc_i32()?)
            }
            Some(biomes)
        } else {
            None
        };
        let buffer_size = buf.get_mc_var_int()? as usize;
        if buffer_size > buf.len() || buffer_size > 2097152 {
            return Err(Box::new(IoError::from(ErrorKind::InvalidData)));
        }
        let buffer = buf.split_to(buffer_size);
        let mut blockEntitiesTags: Vec<Nbt> = vec![];
        let blockEntitiesTags_count = buf.get_mc_var_int()?;
        for _ in 0..blockEntitiesTags_count {
            blockEntitiesTags.push(buf.get_mc_nbt()?);
        }
        return Ok(LevelChunkPacket {
            x,
            z,
            availableSections,
            heightmaps,
            biomes,
            buffer,
            blockEntitiesTags,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(LevelChunkPacket {
            x: 12,
            z: -12,
            availableSections: 1,
            heightmaps: Nbt::make_singleton_compound(
                "test".to_string(),
                Nbt::List {
                    item_type: NbtType::Int,
                    children: vec![Nbt::Int(65)],
                },
            ),
            biomes: Some(Box::new(vec![23].repeat(1 << 10))),
            buffer: BytesMut::from(&vec![0x1a, 0x2b, 0x3c][..]),
            blockEntitiesTags: vec![Nbt::make_singleton_compound(
                "test entity".to_string(),
                Nbt::Int(7),
            )],
        })
    }
}
