use crate::network::*;
use crate::packet::*;
use basin2_lib::result::*;
use bytes::BytesMut;

#[derive(PartialEq, Clone, Debug)]
pub struct SetStructureBlockPacket {
    pub pos: BlockPos,
    pub updateType: StructureBlockEntityUpdateType,
    pub mode: StructureMode,
    pub name: String,
    pub offset: BlockPos,
    pub size: BlockPos,
    pub mirror: Mirror,
    pub rotation: Rotation,
    pub data: String,
    pub ignoreEntities: bool,
    pub showAir: bool,
    pub showBoundingBox: bool,
    pub integrity: f32,
    pub seed: i64,
}

fn clamp(value: i8, from: i32, to: i32) -> i32 {
    let value = value as i32;
    return if value < from {
        from
    } else if value > to {
        to
    } else {
        value
    };
}

impl CodablePacket for SetStructureBlockPacket {
    fn encode(self, buf: &mut BytesMut) {
        buf.set_mc_block_pos(self.pos);
        buf.set_mc_var_int(self.updateType as i32);
        buf.set_mc_var_int(self.mode as i32);
        buf.set_mc_string(self.name);
        buf.set_mc_i8(self.offset.x as i8);
        buf.set_mc_i8(self.offset.y as i8);
        buf.set_mc_i8(self.offset.z as i8);
        buf.set_mc_i8(self.size.x as i8);
        buf.set_mc_i8(self.size.y as i8);
        buf.set_mc_i8(self.size.z as i8);
        buf.set_mc_var_int(self.mirror as i32);
        buf.set_mc_var_int(self.rotation as i32);
        buf.set_mc_string(self.data);
        buf.set_mc_f32(self.integrity);
        buf.set_mc_var_long(self.seed);
        let mut flags: u8 = 0;
        if self.ignoreEntities {
            flags |= 1;
        }
        if self.showAir {
            flags |= 2;
        }
        if self.showBoundingBox {
            flags |= 4;
        }
        buf.set_mc_u8(flags);
    }

    fn decode(buf: &mut BytesMut) -> Result<Self>
    where
        Self: Sized,
    {
        let pos = buf.get_mc_block_pos()?;
        let updateType: StructureBlockEntityUpdateType = buf.get_mc_enum()?;
        let mode: StructureMode = buf.get_mc_enum()?;
        let name = buf.get_mc_string(32767)?;
        let offset = BlockPos {
            x: clamp(buf.get_mc_i8()?, -32, 32),
            y: clamp(buf.get_mc_i8()?, -32, 32),
            z: clamp(buf.get_mc_i8()?, -32, 32),
        };
        let size = BlockPos {
            x: clamp(buf.get_mc_i8()?, -32, 32),
            y: clamp(buf.get_mc_i8()?, -32, 32),
            z: clamp(buf.get_mc_i8()?, -32, 32),
        };
        let mirror: Mirror = buf.get_mc_enum()?;
        let rotation: Rotation = buf.get_mc_enum()?;
        let data = buf.get_mc_string(12)?;
        let integrity = buf.get_mc_f32()?;
        let integrity = if integrity < 0.0 {
            0.0
        } else if integrity > 1.0 {
            1.0
        } else {
            integrity
        };
        let seed = buf.get_mc_var_long()?;
        let flags = buf.get_mc_u8()?;
        let ignoreEntities = (flags & 1) > 0;
        let showAir = (flags & 2) > 0;
        let showBoundingBox = (flags & 4) > 0;
        return Ok(SetStructureBlockPacket {
            pos,
            updateType,
            mode,
            name,
            offset,
            size,
            mirror,
            rotation,
            data,
            ignoreEntities,
            showAir,
            showBoundingBox,
            integrity,
            seed,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packet::test::*;

    #[test]
    fn test_cycle() -> Result<()> {
        cycle(SetStructureBlockPacket {
            pos: BlockPos {
                x: -10,
                y: -20,
                z: -30,
            },
            updateType: StructureBlockEntityUpdateType::UpdateData,
            mode: StructureMode::Save,
            name: "structure name".to_string(),
            offset: BlockPos { x: -1, y: 5, z: 2 },
            size: BlockPos { x: 3, y: 3, z: 3 },
            mirror: Mirror::LeftRight,
            rotation: Rotation::Clockwise90,
            data: "some data".to_string(),
            ignoreEntities: true,
            showAir: false,
            showBoundingBox: true,
            integrity: 1.0,
            seed: 6724562345,
        })
    }
}
