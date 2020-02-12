
use crate::packet::*;
use crate::network::*;
use bytes::BytesMut;
use uuid::Uuid;
use crate::result::*;

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

impl CodablePacket for SetStructureBlockPacket {
    fn encode(self, buf: &mut BytesMut) {
        /* TODO: NOT FOUND */
    }

    fn decode(buf: &mut BytesMut) -> Result<Self> where Self: Sized {
        let pos = buf.get_mc_block_pos()?;
        // TODO: UNKNOWN: this.updateType = (StructureBlockEntity.UpdateType)var1.readEnum(StructureBlockEntity.UpdateType.class);
        // TODO: UNKNOWN: this.mode = (StructureMode)var1.readEnum(StructureMode.class);
        let name = buf.get_mc_string_bounded(32767)?;
        // TODO: UNKNOWN: this.offset = new BlockPos(Mth.clamp(var1.readByte(), -32, 32), Mth.clamp(var1.readByte(), -32, 32), Mth.clamp(var1.readByte(), -32, 32));
        // TODO: UNKNOWN: this.size = new BlockPos(Mth.clamp(var1.readByte(), 0, 32), Mth.clamp(var1.readByte(), 0, 32), Mth.clamp(var1.readByte(), 0, 32));
        // TODO: UNKNOWN: this.mirror = (Mirror)var1.readEnum(Mirror.class);
        // TODO: UNKNOWN: this.rotation = (Rotation)var1.readEnum(Rotation.class);
        let data = buf.get_mc_string_bounded(12)?;
        // TODO: UNKNOWN: this.integrity = Mth.clamp(var1.readFloat(), 0.0F, 1.0F);
        let seed = buf.get_mc_i64()?;
        // TODO: UNKNOWN: byte var2 = var1.readByte();
        // TODO: EXTRA: this.ignoreEntities = (var2 & 1) != 0;
        // TODO: EXTRA: this.showAir = (var2 & 2) != 0;
        // TODO: EXTRA: this.showBoundingBox = (var2 & 4) != 0;
        return Ok(SetStructureBlockPacket { pos, updateType, mode, name, offset, size, mirror, rotation, data, ignoreEntities, showAir, showBoundingBox, integrity, seed });
    }
}
