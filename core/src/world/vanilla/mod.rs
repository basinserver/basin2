use std::collections::HashMap;
use super::chunk::*;
use super::*;
use std::sync::{Arc, Weak};
use memmap::{MmapOptions, Mmap};
use std::fs::File;
use crate::result::*;
use bytes::{ Bytes, BytesMut };
use bytes::buf::Buf;
use flate2::write::{GzDecoder, ZlibDecoder};
use std::io::Write;
use basin2_protocol::Nbt;
use chashmap::CHashMap;
use super::block::Block;
use linked_hash_map::LinkedHashMap;
use bitvec::prelude::*;
use log::*;
use super::tile_entity::*;
use basin2_protocol::network::BlockPos;

struct VanillaRegion {
    x: i32,
    z: i32,
    file: File,
    mmap: Option<Mmap>,
}

impl VanillaRegion {
    fn new(x: i32, z: i32, file: File) -> Result<VanillaRegion> {
        let mut region = VanillaRegion {
            x,
            z,
            file,
            mmap: None,
        };
        region.mmap = Some(unsafe { MmapOptions::new().map(&region.file)? });
        Ok(region)
    }

    // coordinates within region
    fn load_chunk(&self, x: u32, z: u32) -> Result<Option<Nbt>> {
        if x >= 32 || z >= 32 {
            panic!("attempted to load a chunk out of bounds: {}, {}", x, z);
        }
        let file_offset = ((x + z * 32) * 4) as usize;
        let mmap = self.mmap.as_ref().unwrap();
        let chunk_offset = &mmap[file_offset..file_offset + 4];
        let chunk_file_offset = (((chunk_offset[0] as u32) << 16) |
            ((chunk_offset[1] as u32) << 8) |
            (chunk_offset[2] as u32)) * 4096;
        let chunk_file_size = chunk_offset[3] as u32 * 4096;
        if chunk_file_offset == 0 || chunk_file_size == 0 {
            return Ok(None);
        }
        // we dont really care about the timestamp right now
        let exact_chunk_size = &mmap[chunk_file_offset as usize..chunk_file_offset as usize + 4];
        let exact_chunk_size = Bytes::from(exact_chunk_size.to_vec()).get_u32();
        if exact_chunk_size > chunk_file_size {
            return Err(basin_err!("chunk has invalid size (greater than allocated sector count), cannot load"));
        }
        let compression_scheme = mmap[chunk_file_offset as usize + 4];
        let compressed_chunk = &mmap[chunk_file_offset as usize + 5..chunk_file_offset as usize + exact_chunk_size as usize + 4];
        let mut decompressed_chunk = match compression_scheme {
            0 => BytesMut::from(compressed_chunk),
            1 => {
                let mut deflater = GzDecoder::new(vec![]);
                deflater.write_all(compressed_chunk)?;
                BytesMut::from(&deflater.finish()?[..])
            },
            2 => {
                let mut deflater = ZlibDecoder::new(vec![]);
                deflater.write_all(compressed_chunk)?;
                BytesMut::from(&deflater.finish()?[..])
            },
            _ => return Err(basin_err!("invalid compression_scheme for chunk: {}", compression_scheme))
        };
        Ok(Some(Nbt::parse(&mut decompressed_chunk)?))
    }
}

pub struct VanillaChunk {
    region: Arc<VanillaRegion>,
    x: i32,
    z: i32,
    biomes: [i32; 1024],
    sections: [Option<VanillaChunkSection>; 16],
    motion_blocking_heightmap: [u16; 256],
    world_surface_heightmap: [u16; 256],
    block_entities: CHashMap<i64, TileEntity>,
}

// we expand out from the palette in loaded to chunks to simplify and cheapen logic for block access/update.
// if needed, we can go back to the basin v1 approach of keeping it compressed in memory.
struct VanillaChunkSection {
    blocks: [u16; 16*16*16],
    block_light: [u8; 16*16*16/2],
    sky_light: Option<[u8; 16*16*16/2]>,
    palette_bits: u8,
    palette: Option<Vec<u16>>,
}

fn i64_to_u8_slice(input: &Vec<i64>) -> Vec<u8> {
    unsafe { input.align_to().1.to_vec() }
}

fn parse_heightmap(children: &LinkedHashMap<String, Nbt>, name: &'static str) -> Result<[u16; 256]> {
    Ok(match children.get(name) {
        Some(Nbt::LongArray(longs)) => {
            let longs = i64_to_u8_slice(longs);
            let slice = BitSlice::from_slice(&longs);
            let mut heights: [u16; 256] = [0; 256];
            heights.clone_from_slice(&slice.chunks(9).map(|chunk: &BitSlice<Local, u8>| chunk.load_be::<u16>()).collect::<Vec<u16>>()[0..256]);
            heights
        }
        _ => return Err(basin_err!("missing {} in chunk's Heightmaps", name))
    })
}

impl VanillaChunk {

    fn from_nbt_level(region: Arc<VanillaRegion>, level: &LinkedHashMap<String, Nbt>, x: i32, z: i32) -> Result<VanillaChunk> {
        let xPos = level.get("xPos");
        match xPos {
            Some(Nbt::Int(nbt_x)) => {
                if *nbt_x != x {
                    return Err(basin_err!("mismatched chunk X coordinate with expectation"));
                }
            }
            _ => return Err(basin_err!("missing xPos in chunk"))
        }
        let zPos = level.get("zPos");
        match zPos {
            Some(Nbt::Int(nbt_z)) => {
                if *nbt_z != z {
                    return Err(basin_err!("mismatched chunk Z coordinate with expectation"));
                }
            }
            _ => return Err(basin_err!("missing zPos in chunk"))
        }
        let biomes = match level.get("Biomes") {
            Some(Nbt::IntArray(biomes)) => {
                let mut biomes_array: [i32; 1024] = [0; 1024];
                biomes_array.clone_from_slice(&biomes[0..1024]);
                biomes_array
            },
            _ => return Err(basin_err!("missing Biomes in chunk"))
        };
        if biomes.len() != 1024 {
            return Err(basin_err!("Biomes is corrupt in chunk (expected 1024 entires, found {})", biomes.len()));
        }

        // kinda hacky way to avoid implementing copy on sections
        let mut sections: [Option<VanillaChunkSection>; 16] = [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None];
        match level.get("Sections") {
            Some(Nbt::List { children, .. }) => {
                for child in children {
                    match child {
                        Nbt::Compound { children } => {
                            let yIndex = match children.get("Y") {
                                Some(Nbt::Byte(b)) => b,
                                _ => return Err(basin_err!("missing Sections in chunk"))
                            };
                            // TODO: how is the global palette handled here?
                            let palette = match children.get("Palette") {
                                Some(Nbt::List { children, .. }) => {
                                    // TODO: we need to setup the block registry to continue here
                                    // https://minecraft.gamepedia.com/Chunk_format
                                    vec![]
                                },
                                _ => return Err(basin_err!("missing Palette in chunk section"))
                            };
                            let mut palette_bits = ((palette.len() - 1) as f64).log2().ceil() as u8;
                            if palette_bits < 4 {
                                palette_bits = 4;
                            }
                            let blocks = match children.get("BlockStates") {
                                Some(Nbt::LongArray(longs)) => {
                                    let blocks_raw = i64_to_u8_slice(longs);
                                    let blocks_raw = BitSlice::from_slice(&blocks_raw);
                                    let mut blocks: [u16; 4096] = [0; 4096];
                                    blocks.clone_from_slice(&blocks_raw.chunks(palette_bits as usize).map(|chunk: &BitSlice<Local, u8>| chunk.load_be::<u16>()).collect::<Vec<u16>>()[0..4096]);
                                    blocks
                                }
                                _ => return Err(basin_err!("missing BlockStates in chunk section"))
                            };
                            let mut block_light: [u8; 2048] = [0; 2048];
                            block_light.clone_from_slice(match children.get("BlockLight") {
                                Some(Nbt::ByteArray(bytes)) => {
                                    &bytes[0..2048]
                                },
                                _ => return Err(basin_err!("missing BlockLight in chunk section"))
                            });
                            let sky_light = match children.get("SkyLight") {
                                Some(Nbt::ByteArray(bytes)) => {
                                    let mut sky_light: [u8; 2048] = [0; 2048];
                                    block_light.clone_from_slice(&bytes[0..2048]);
                                    Some(sky_light)
                                },
                                _ => None,
                            };
                            sections[*yIndex as usize] = Some(VanillaChunkSection {
                                blocks,
                                block_light,
                                sky_light,
                                palette_bits,
                                palette: Some(palette),
                            });
                        }
                        _ => return Err(basin_err!("missing Sections in chunk"))
                    }
                }
            },
            _ => return Err(basin_err!("missing Sections in chunk"))
        }
        let (motion_blocking_heightmap, world_surface_heightmap) =
        match level.get("Heightmaps") {
            Some(Nbt::Compound { children }) => {
                (
                    parse_heightmap(&children, "MOTION_BLOCKING")?,
                    parse_heightmap(&children, "WORLD_SURFACE")?,
                )
            }
            _ => return Err(basin_err!("missing Heightmaps in chunk"))
        };
        let mut tile_entities: CHashMap<i64, TileEntity> = CHashMap::new();
        // TODO: entities
        match level.get("TileEntities") {
            Some(Nbt::List { children, .. }) => {
                for child in children {
                    match child {
                        Nbt::Compound { children } => {
                            let id = match children.get("id") {
                                Some(Nbt::Str(string)) => string,
                                _ => {
                                    warn!("nbt entry missing id, skipping...");
                                    continue;
                                }
                            };
                            let x = match children.get("x") {
                                Some(Nbt::Int(x)) => x,
                                _ => {
                                    warn!("nbt entry missing x coordinate, skipping...");
                                    continue;
                                }
                            };
                            let y = match children.get("y") {
                                Some(Nbt::Int(x)) => x,
                                _ => {
                                    warn!("nbt entry missing y coordinate, skipping...");
                                    continue;
                                }
                            };
                            let z = match children.get("z") {
                                Some(Nbt::Int(x)) => x,
                                _ => {
                                    warn!("nbt entry missing z coordinate, skipping...");
                                    continue;
                                }
                            };
                            tile_entities.insert(BlockPos { x: *x, y: *y, z: *z }.into(), TileEntity {
                                id: id.clone(),
                                x: *x,
                                y: *y,
                                z: *z,
                                data: child.clone(),
                            });
                        }
                        _ => (),
                    }
                }
            }
            _ => (),
        }
        // TODO: tile ticks
        Ok(VanillaChunk {
            region,
            x,
            z,
            biomes,
            sections,
            motion_blocking_heightmap,
            world_surface_heightmap,
            block_entities: tile_entities,
        })
    }

    fn from_nbt(region: Arc<VanillaRegion>, nbt: Nbt, x: i32, z: i32) -> Result<VanillaChunk> {
        //TODO: check data version
        match nbt {
            Nbt::Compound { children } => {
                match children.get("Level") {
                    Some(Nbt::Compound { children: level }) => {
                        return VanillaChunk::from_nbt_level(region, level, x, z);
                    }
                    _ => return Err(basin_err!("invalid chunk nbt format"))
                }
            }
            _ => return Err(basin_err!("invalid chunk nbt format"))
        }
    }

    fn new(region: Arc<VanillaRegion>, x: i32, z: i32) -> VanillaChunk {
        VanillaChunk {
            region,
            x,
            z,
            biomes: [0; 1024],
            sections: [None, None, None, None, None, None, None, None, None, None, None, None, None, None, None, None],
            motion_blocking_heightmap: [0; 256],
            world_surface_heightmap: [0; 256],
            block_entities: CHashMap::new(),
        }
    }
}

impl ChunkT for VanillaChunk {
    fn get_block(&self, x: i32, y: i32, z: i32) -> Block {
        return Block {};
    }

    fn set_block(&self, x: i32, y: i32, z: i32, block: Block) {
        
    }

    fn get_x(&self) -> i32 {
        return self.x;
    }

    fn get_z(&self) -> i32 {
        return self.z;
    }
}

pub struct VanillaWorld {
    directory: String,
    regions: CHashMap<u64, Arc<VanillaRegion>>,
    loaded_chunks: CHashMap<u64, Weak<VanillaChunk>>,
    
}

impl VanillaWorld {
    fn new(directory: String) -> Result<VanillaWorld> {

        Err(basin_err!("nyi"))
    }

    fn new_region(&self, x: i32, z: i32) -> Result<Arc<VanillaRegion>> {

        Err(basin_err!("nyi"))
    }
}

impl WorldT for VanillaWorld {
    fn get_chunk(&self, x: i32, z: i32) -> Result<Chunk> {
        let id = chunk_id(x, z);
        let preloaded = self.loaded_chunks.get(&id);
        match preloaded {
            Some(chunk) => {
                match chunk.upgrade() {
                    Some(chunk) => return Ok(chunk),
                    _ => (),
                }
            }
            _ => (),
        }
        let region_id = chunk_id(x >> 5, z >> 5);
        let region = self.regions.get(&region_id);
        let region_chunk = match &region {
            Some(region) => {
                region.load_chunk((x & 31) as u32, (z & 31) as u32)?
            },
            None => {
                None
            },
        };
        let region = match region {
            Some(region) => region.clone(),
            None => self.new_region(x >> 5, z >> 5)?,
        };
        let loaded_chunk = Arc::new(match region_chunk {
            Some(nbt) => VanillaChunk::from_nbt(region, nbt, x, z)?,
            None => VanillaChunk::new(region, x, z),
        });
        self.loaded_chunks.insert(chunk_id(loaded_chunk.x, loaded_chunk.z), Arc::downgrade(&loaded_chunk));
        Ok(loaded_chunk)
    }

    fn save(&self) {

    }
}