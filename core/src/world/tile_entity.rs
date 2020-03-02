use basin2_lib::Nbt;

pub struct TileEntity {
    pub id: String,
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub data: Nbt,
}