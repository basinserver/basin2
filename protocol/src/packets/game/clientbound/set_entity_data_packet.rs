use crate::network::*;
use crate::packet::*;
use crate::result::*;
use bytes::BytesMut;

impl EntityMetadata {
    pub fn serialize(self, buf: &mut BytesMut) {
        use EntityMetadata::*;

        match self {
            Byte(value) => {
                buf.set_mc_u8(0);
                buf.set_mc_i8(value);
            }
            Int(value) => {
                buf.set_mc_u8(1);
                buf.set_mc_var_int(value);
            }
            Float(value) => {
                buf.set_mc_u8(2);
                buf.set_mc_f32(value);
            }
            Str(value) => {
                buf.set_mc_u8(3);
                buf.set_mc_string(value);
            }
            Component(component) => {
                buf.set_mc_u8(4);
                buf.set_mc_chat_component(component);
            }
            OptionalComponent(Some(component)) => {
                buf.set_mc_u8(5);
                buf.set_mc_bool(true);
                buf.set_mc_chat_component(component);
            }
            OptionalComponent(None) => {
                buf.set_mc_u8(5);
                buf.set_mc_bool(false);
            }
            ItemStack(item) => {
                buf.set_mc_u8(6);
                buf.set_mc_item_stack(item);
            }
            Boolean(value) => {
                buf.set_mc_u8(7);
                buf.set_mc_bool(value);
            }
            Rotations { x, y, z } => {
                buf.set_mc_u8(8);
                buf.set_mc_f32(x);
                buf.set_mc_f32(y);
                buf.set_mc_f32(z);
            }
            BlockPos(pos) => {
                buf.set_mc_u8(9);
                buf.set_mc_block_pos(pos);
            }
            OptionalBlockPos(Some(pos)) => {
                buf.set_mc_u8(10);
                buf.set_mc_bool(true);
                buf.set_mc_block_pos(pos);
            }
            OptionalBlockPos(None) => {
                buf.set_mc_u8(10);
                buf.set_mc_bool(false);
            }
            Direction(direction) => {
                buf.set_mc_u8(11);
                buf.set_mc_var_int(direction as i32);
            }
            OptionalUuid(Some(uuid)) => {
                buf.set_mc_u8(12);
                buf.set_mc_bool(true);
                buf.set_mc_uuid(uuid);
            }
            OptionalUuid(None) => {
                buf.set_mc_u8(12);
                buf.set_mc_bool(false);
            }
            BlockState(state) => {
                buf.set_mc_u8(13);
                buf.set_mc_var_int(state);
            }
            CompoundTag(nbt) => {
                buf.set_mc_u8(14);
                buf.set_mc_nbt(nbt);
            }
            Particle(particle_options) => {
                buf.set_mc_u8(15);
                particle_options.serialize(buf);
            }
            VillagerData {
                villager_type,
                villager_profession,
                level,
            } => {
                buf.set_mc_u8(16);
                buf.set_mc_var_int(villager_type as i32);
                buf.set_mc_var_int(villager_profession as i32);
                buf.set_mc_var_int(level);
            }
            OptionalUnsignedInt(Some(value)) => {
                buf.set_mc_u8(17);
                buf.set_mc_var_int(value + 1);
            }
            OptionalUnsignedInt(None) => {
                buf.set_mc_u8(17);
                buf.set_mc_var_int(0);
            }
            Pose(pose) => {
                buf.set_mc_u8(18);
                buf.set_mc_var_int(pose as i32);
            }
        }
    }

    pub fn parse(buf: &mut BytesMut) -> Result<EntityMetadata> {
        use EntityMetadata::*;
        let tag_type = buf.get_mc_u8()?;
        Ok(match tag_type {
            0 => Byte(buf.get_mc_i8()?),
            1 => Int(buf.get_mc_var_int()?),
            2 => Float(buf.get_mc_f32()?),
            3 => Str(buf.get_mc_string(32767)?),
            4 => Component(buf.get_mc_chat_component()?),
            5 => {
                if buf.get_mc_bool()? {
                    OptionalComponent(Some(buf.get_mc_chat_component()?))
                } else {
                    OptionalComponent(None)
                }
            }
            6 => ItemStack(buf.get_mc_item_stack()?),
            7 => Boolean(buf.get_mc_bool()?),
            8 => Rotations {
                x: buf.get_mc_f32()?,
                y: buf.get_mc_f32()?,
                z: buf.get_mc_f32()?,
            },
            9 => BlockPos(buf.get_mc_block_pos()?),
            10 => {
                if buf.get_mc_bool()? {
                    OptionalBlockPos(Some(buf.get_mc_block_pos()?))
                } else {
                    OptionalBlockPos(None)
                }
            }
            11 => Direction(buf.get_mc_enum()?),
            12 => {
                if buf.get_mc_bool()? {
                    OptionalUuid(Some(buf.get_mc_uuid()?))
                } else {
                    OptionalUuid(None)
                }
            }
            13 => BlockState(buf.get_mc_var_int()?),
            14 => CompoundTag(buf.get_mc_nbt()?),
            15 => Particle(ParticleOptions::parse(buf.get_mc_enum()?, buf)?),
            16 => VillagerData {
                villager_type: buf.get_mc_enum()?,
                villager_profession: buf.get_mc_enum()?,
                level: buf.get_mc_var_int()?,
            },
            17 => {
                let raw = buf.get_mc_var_int()?;
                OptionalUnsignedInt(if raw == 0 { None } else { Some(raw - 1) })
            }
            18 => Pose(buf.get_mc_enum()?),
            _ => return Err(Box::new(IoError::from(ErrorKind::InvalidData))),
        })
    }
}

#[derive(PartialEq, Clone, Debug)]
pub struct SetEntityDataPacket {
    pub id: i32,
    pub metadata: Vec<EntityMetadataItem>,
}

impl CodablePacket for SetEntityDataPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_var_int(self.id);
        for item in self.metadata {
            buf.set_mc_u8(item.id);
            item.data.serialize(buf);
        }
        buf.set_mc_u8(255);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let id = buf.get_mc_var_int()?;
        let mut metadata: Vec<EntityMetadataItem> = vec![];
        loop {
            let local_id = buf.get_mc_u8()?;
            if local_id == 255 {
                break;
            }
            metadata.push(EntityMetadataItem {
                id: local_id,
                data: EntityMetadata::parse(buf)?,
            })
        }
        return Ok(SetEntityDataPacket { id, metadata });
    }
}
