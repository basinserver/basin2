use linked_hash_map::LinkedHashMap;
use bytes::BytesMut;
use crate::network::*;
use crate::result::*;


enum_from_primitive! {
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum NbtType {
    End,
    Byte,
    Short,
    Int,
    Long,
    Float,
    Double,
    ByteArray,
    Str,
    List,
    Compound,
    IntArray,
    LongArray,
}
}

pub enum Nbt {
    End,
    Byte(i8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    Str(String),
    List { item_type: NbtType, children: Vec<Nbt> },
    Compound { children: LinkedHashMap<String, Nbt> },
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Nbt {

    pub fn nbt_type(&self) -> NbtType {
        use Nbt::*;
        match self {
            End => NbtType::End,
            Byte(..) => NbtType::Byte,
            Short(..) => NbtType::Short,
            Int(..) => NbtType::Int,
            Long(..) => NbtType::Long,
            Float(..) => NbtType::Float,
            Double(..) => NbtType::Double,
            ByteArray(..) => NbtType::ByteArray,
            Str(..) => NbtType::Str,
            List { .. } => NbtType::List,
            Compound { .. } => NbtType::Compound,
            IntArray(..) => NbtType::IntArray,
            LongArray(..) => NbtType::LongArray,
        }
    }

    pub fn parse(buf: &mut BytesMut) -> Result<Nbt> {
        let (name, nbt) = Nbt::parse_item(buf)?;
        if name.is_none() {
            return invalidData();
        }
        let mut children = LinkedHashMap::new();
        children.insert(name.unwrap().to_string(), nbt);
        Ok(Nbt::Compound { children })
    }

    fn parse_item(buf: &mut BytesMut) -> Result<(Option<String>, Nbt)> {
        let nbt_type: NbtType = buf.get_mc_enum_u8()?;
        let name = match nbt_type {
            NbtType::End => None,
            _ => {
                let name_length = buf.get_mc_u16()? as usize;
                if buf.len() < name_length {
                    return invalidData();
                }
                let bytes = buf.split_to(name_length).to_vec();
                let name = &*String::from_utf8_lossy(&bytes);
                Some(name.to_string())
            }
        };
        Ok((name, Nbt::parse_list(buf, nbt_type)?))
    }

    fn parse_list(buf: &mut BytesMut, nbt_type: NbtType) -> Result<Nbt> {
        use NbtType::*;
        Ok(match nbt_type {
            End => {
                Nbt::End
            },
            Byte => {
                Nbt::Byte(buf.get_mc_i8()?)
            },
            Short => {
                Nbt::Short(buf.get_mc_i16()?)
            },
            Int => {
                Nbt::Int(buf.get_mc_i32()?)
            },
            Long => {
                Nbt::Long(buf.get_mc_i64()?)
            },
            Float => {
                Nbt::Float(buf.get_mc_f32()?)
            },
            Double => {
                Nbt::Double(buf.get_mc_f64()?)
            },
            ByteArray => {
                let length = buf.get_mc_i32()? as usize;
                if length > buf.len() {
                    return invalidData();
                }
                Nbt::ByteArray(buf.split_to(length).to_vec())
            },
            Str => {
                let string_length = buf.get_mc_u16()? as usize;
                if buf.len() < string_length {
                    return invalidData();
                }
                let bytes = buf.split_to(string_length).to_vec();
                let string = &*String::from_utf8_lossy(&bytes);
                Nbt::Str(string.to_string())
            },
            List => {
                let item_type: NbtType = buf.get_mc_enum_u8()?;
                let count = buf.get_mc_i32()? as usize;
                let mut children: Vec<Nbt> = vec![];
                for _ in 0..count {
                    let item = Nbt::parse_list(buf, nbt_type)?;
                    match item {
                        Nbt::End => break, // should never happen
                        _ => (),
                    }
                    children.push(item);
                }
                Nbt::List { item_type, children }
            },
            Compound => {
                let mut children = LinkedHashMap::new();
                loop {
                    let (name, item) = Nbt::parse_item(buf)?;
                    match item {
                        Nbt::End => break,
                        _ => (),
                    }
                    children.insert(name.expect("name not found and should have been").to_string(), item);
                }
                Nbt::Compound { children }
            },
            IntArray => {
                let length = buf.get_mc_i32()? as usize;
                if length * 4 > buf.len() {
                    return invalidData();
                }
                Nbt::IntArray(unsafe { buf.split_to(length * 4).align_to::<i32>() }.1.to_vec())
            },
            LongArray => {
                let length = buf.get_mc_i32()? as usize;
                if length * 8 > buf.len() {
                    return invalidData();
                }
                Nbt::LongArray(unsafe { buf.split_to(length * 8).align_to::<i64>() }.1.to_vec())
            },
        })
    }

    pub fn serialize(self, buf: &mut BytesMut) {
        match self {
            Nbt::Compound { .. } => (),
            _ => panic!("attempted to serialize non-compound!"),
        }
        self.serialize_list(buf);
    }

    fn serialize_item(self, name: Option<&str>, buf: &mut BytesMut) {
        buf.set_mc_u8(self.nbt_type() as u8);
        match name {
            Some(name) => {
                let bytes = name.as_bytes();
                buf.set_mc_i16(bytes.len() as i16);
                buf.extend(bytes);
            },
            _ => (),
        }
        self.serialize_list(buf);
    }

    fn serialize_list(self, buf: &mut BytesMut) {
        use Nbt::*;
        match self {
            End => {
            },
            Byte(value) => {
                buf.set_mc_i8(value);
            },
            Short(value) => {
                buf.set_mc_i16(value);
            },
            Int(value) => {
                buf.set_mc_i32(value);
            },
            Long(value) => {
                buf.set_mc_i64(value);
            },
            Float(value) => {
                buf.set_mc_f32(value);
            },
            Double(value) => {
                buf.set_mc_f64(value);
            },
            ByteArray(value) => {
                buf.set_mc_i32(value.len() as i32);
                buf.extend(value);
            },
            Str(value) => {
                let bytes = value.as_bytes();
                buf.set_mc_i16(bytes.len() as i16);
                buf.extend(bytes);
            },
            List { item_type, children } => {
                buf.set_mc_u8(item_type as u8);
                buf.set_mc_i32(children.len() as i32);
                for child in children {
                    child.serialize_list(buf);
                }
            },
            Compound { children } => {
                for (name, item) in children {
                    item.serialize_item(Some(&*name), buf);
                }
                End.serialize_item(None, buf);
            },
            IntArray(value) => {
                buf.set_mc_i32(value.len() as i32);
                buf.extend_from_slice(unsafe { value.align_to::<u8>() }.1);
            },
            LongArray(value) => {
                buf.set_mc_i32(value.len() as i32);
                buf.extend_from_slice(unsafe { value.align_to::<u8>() }.1);
            },
        }
    }
}