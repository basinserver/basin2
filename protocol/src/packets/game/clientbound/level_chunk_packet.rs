
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use crate::result::*;
use crate::nbt::Nbt;

pub struct LevelChunkPacket {
    pub x: i32,
    pub z: i32,
    pub availableSections: i32,
    pub heightmaps: Nbt,
    pub biomes: Option<Vec<i32>>,
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
                for biome in biomes {
                    buf.set_mc_i32(biome);
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

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let x = buf.get_mc_i32()?;
        let z = buf.get_mc_i32()?;
        let has_biomes = buf.get_mc_bool()?;
        let availableSections = buf.get_mc_var_int()?;
        let heightmaps = buf.get_mc_nbt()?;
        let biomes =
            if has_biomes {
                let mut biomes: Vec<i32> = vec![];
                for _ in 0..(1 << 18) {
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
        return Ok(LevelChunkPacket { x, z, availableSections, heightmaps, biomes, buffer, blockEntitiesTags });
    }
}
