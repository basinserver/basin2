use crate::result::*;
use bytes::BytesMut;
use linked_hash_map::LinkedHashMap;
use crate::{ mcproto, McProtoBase };
use crate::basin_err;

enum_from_primitive! {
#[derive(Clone, Copy, PartialEq, Debug)]
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

#[derive(PartialEq, Clone, Debug)]
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
    List {
        item_type: NbtType,
        children: Vec<Nbt>,
    },
    Compound {
        children: LinkedHashMap<String, Nbt>,
    },
    IntArray(Vec<i32>),
    LongArray(Vec<i64>),
}

impl Nbt {
    pub fn make_singleton_compound(key: String, value: Nbt) -> Nbt {
        let mut children = LinkedHashMap::new();
        children.insert(key, value);
        Nbt::Compound { children }
    }

    pub fn make_compound(values: Vec<(String, Nbt)>) -> Nbt {
        let mut children = LinkedHashMap::new();
        for (key, value) in values {
            children.insert(key, value);
        }
        Nbt::Compound { children }
    }

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
        let direct_nbt = Nbt::parse_list(buf, NbtType::Compound)?;
        match direct_nbt {
            Nbt::Compound { children } if children.len() == 1 && children.contains_key("") => {
                Ok(children[""].clone())
            },
            _ => Ok(direct_nbt)
        }
    }

    fn parse_item(buf: &mut BytesMut) -> Result<(Option<String>, Nbt)> {
        let nbt_type: NbtType = if buf.len() == 0 {
            NbtType::End
        } else {
            buf.get_mc_enum_u8()?
        };
        let name = match nbt_type {
            NbtType::End => None,
            _ => {
                let name_length = buf.get_mc_u16()? as usize;
                if buf.len() < name_length {
                    return mcproto::invalid_data();
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
            End => Nbt::End,
            Byte => Nbt::Byte(buf.get_mc_i8()?),
            Short => Nbt::Short(buf.get_mc_i16()?),
            Int => Nbt::Int(buf.get_mc_i32()?),
            Long => Nbt::Long(buf.get_mc_i64()?),
            Float => Nbt::Float(buf.get_mc_f32()?),
            Double => Nbt::Double(buf.get_mc_f64()?),
            ByteArray => {
                let length = buf.get_mc_i32()? as usize;
                if length > buf.len() {
                    return mcproto::invalid_data::<Nbt>();
                }
                Nbt::ByteArray(buf.split_to(length).to_vec())
            }
            Str => {
                let string_length = buf.get_mc_u16()? as usize;
                if buf.len() < string_length {
                    return mcproto::invalid_data();
                }
                let bytes = buf.split_to(string_length).to_vec();
                let string = &*String::from_utf8_lossy(&bytes);
                Nbt::Str(string.to_string())
            }
            List => {
                let item_type: NbtType = buf.get_mc_enum_u8()?;
                let count = buf.get_mc_i32()? as usize;
                let mut children: Vec<Nbt> = vec![];
                for _ in 0..count {
                    let item = Nbt::parse_list(buf, item_type)?;
                    match item {
                        Nbt::End => break, // should never happen
                        _ => (),
                    }
                    children.push(item);
                }
                Nbt::List {
                    item_type,
                    children,
                }
            }
            Compound => {
                let mut children = LinkedHashMap::new();
                loop {
                    let (name, item) = Nbt::parse_item(buf)?;
                    match item {
                        Nbt::End => break,
                        _ => (),
                    }
                    children.insert(
                        name.expect("name not found and should have been")
                            .to_string(),
                        item,
                    );
                }
                Nbt::Compound { children }
            }
            IntArray => {
                let length = buf.get_mc_i32()? as usize;
                if length * 4 > buf.len() {
                    return mcproto::invalid_data();
                }
                Nbt::IntArray(buf.read_primitive_slice(length)?)
            }
            LongArray => {
                let length = buf.get_mc_i32()? as usize;
                if length * 8 > buf.len() {
                    return mcproto::invalid_data();
                }
                Nbt::LongArray(buf.read_primitive_slice(length)?)
            }
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
            }
            _ => (),
        }
        self.serialize_list(buf);
    }

    fn serialize_list(self, buf: &mut BytesMut) {
        use Nbt::*;
        match self {
            End => {}
            Byte(value) => {
                buf.set_mc_i8(value);
            }
            Short(value) => {
                buf.set_mc_i16(value);
            }
            Int(value) => {
                buf.set_mc_i32(value);
            }
            Long(value) => {
                buf.set_mc_i64(value);
            }
            Float(value) => {
                buf.set_mc_f32(value);
            }
            Double(value) => {
                buf.set_mc_f64(value);
            }
            ByteArray(value) => {
                buf.set_mc_i32(value.len() as i32);
                buf.extend(value);
            }
            Str(value) => {
                let bytes = value.as_bytes();
                buf.set_mc_i16(bytes.len() as i16);
                buf.extend(bytes);
            }
            List {
                item_type,
                children,
            } => {
                buf.set_mc_u8(item_type as u8);
                buf.set_mc_i32(children.len() as i32);
                for child in children {
                    child.serialize_list(buf);
                }
            }
            Compound { children } => {
                for (name, item) in children {
                    item.serialize_item(Some(&*name), buf);
                }
                End.serialize_item(None, buf);
            }
            IntArray(value) => {
                buf.set_mc_i32(value.len() as i32);
                buf.write_primitive_slice(&value[..]);
            }
            LongArray(value) => {
                buf.set_mc_i32(value.len() as i32);
                buf.write_primitive_slice(&value[..]);
            }
        }
    }

    pub fn child(&self, key: &str) -> Result<&Nbt> {
        match self {
            Nbt::Compound { children } => {
                children.get(key).map(|item| Ok(item)).unwrap_or(Err(basin_err!("could not find key {} in nbt", key)))
            },
            _ => Err(basin_err!("could not get key {} from non-compound nbt tag", key)),
        }
    }

    pub fn unwrap_i8(&self) -> Result<i8> {
        match self {
            Nbt::Byte(value) => Ok(*value),
            _ => Err(basin_err!("invalid nbt tag type, expected Byte got {:?}", self.nbt_type())),
        }
    }

    pub fn unwrap_i16(&self) -> Result<i16> {
        match self {
            Nbt::Short(value) => Ok(*value),
            _ => Err(basin_err!("invalid nbt tag type, expected Short got {:?}", self.nbt_type())),
        }
    }

    pub fn unwrap_i32(&self) -> Result<i32> {
        match self {
            Nbt::Int(value) => Ok(*value),
            _ => Err(basin_err!("invalid nbt tag type, expected Int got {:?}", self.nbt_type())),
        }
    }

    pub fn unwrap_i64(&self) -> Result<i64> {
        match self {
            Nbt::Long(value) => Ok(*value),
            _ => Err(basin_err!("invalid nbt tag type, expected Long got {:?}", self.nbt_type())),
        }
    }

    pub fn unwrap_f32(&self) -> Result<f32> {
        match self {
            Nbt::Float(value) => Ok(*value),
            _ => Err(basin_err!("invalid nbt tag type, expected Float got {:?}", self.nbt_type())),
        }
    }

    pub fn unwrap_f64(&self) -> Result<f64> {
        match self {
            Nbt::Double(value) => Ok(*value),
            _ => Err(basin_err!("invalid nbt tag type, expected Double got {:?}", self.nbt_type())),
        }
    }

    pub fn unwrap_bytes(&self) -> Result<&[u8]> {
        match self {
            Nbt::ByteArray(value) => Ok(value),
            _ => Err(basin_err!("invalid nbt tag type, expected ByteArray got {:?}", self.nbt_type())),
        }
    }

    pub fn unwrap_str(&self) -> Result<&str> {
        match self {
            Nbt::Str(value) => Ok(value),
            _ => Err(basin_err!("invalid nbt tag type, expected Str got {:?}", self.nbt_type())),
        }
    }

    pub fn unwrap_compound(&self) -> Result<&LinkedHashMap<String, Nbt>> {
        match self {
            Nbt::Compound { children } => Ok(children),
            _ => Err(basin_err!("invalid nbt tag type, expected Compound got {:?}", self.nbt_type())),
        }
    }

    pub fn unwrap_list(&self) -> Result<&[Nbt]> {
        match self {
            Nbt::List { children, .. } => Ok(children),
            _ => Err(basin_err!("invalid nbt tag type, expected List got {:?}", self.nbt_type())),
        }
    }

    pub fn unwrap_ints(&self) -> Result<&[i32]> {
        match self {
            Nbt::IntArray(value) => Ok(value),
            _ => Err(basin_err!("invalid nbt tag type, expected IntArray got {:?}", self.nbt_type())),
        }
    }

    pub fn unwrap_longs(&self) -> Result<&[i64]> {
        match self {
            Nbt::LongArray(value) => Ok(value),
            _ => Err(basin_err!("invalid nbt tag type, expected LongArray got {:?}", self.nbt_type())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn cycle(nbt: Nbt) -> Result<()> {
        let mut buf = BytesMut::new();
        nbt.clone().serialize(&mut buf);
        // let original_buf = buf.clone();
        let decoded = Nbt::parse(&mut buf)?;
        assert_eq!(nbt, decoded);
        Ok(())
    }

    #[test]
    fn test_simple_compound() -> Result<()> {
        cycle(Nbt::make_compound(vec![
            ("byte".to_string(), Nbt::Byte(120)),
            ("short".to_string(), Nbt::Short(12000)),
            ("int".to_string(), Nbt::Int(43563456)),
            ("long".to_string(), Nbt::Long(435643563456)),
            ("float".to_string(), Nbt::Float(345345.345345)),
            ("double".to_string(), Nbt::Double(34532.53456)),
            (
                "byte_array".to_string(),
                Nbt::ByteArray(vec![0x0a, 0x0b, 0x0c]),
            ),
            ("str".to_string(), Nbt::Str("a string".to_string())),
            (
                "int_array".to_string(),
                Nbt::IntArray(vec![2345, 23453245, 3452345, 324523]),
            ),
            (
                "long_array".to_string(),
                Nbt::LongArray(vec![
                    0xffffffff,
                    345643564356,
                    43563456,
                    456456456,
                    456456456345,
                    56345634563456,
                ]),
            ),
        ]))
    }

    #[test]
    fn test_nested_compound() -> Result<()> {
        cycle(Nbt::make_compound(vec![
            (
                "nest1".to_string(),
                Nbt::make_compound(vec![("int".to_string(), Nbt::Int(43563456))]),
            ),
            ("tail_int".to_string(), Nbt::Int(43563456)),
        ]))
    }

    #[test]
    fn test_nested_list_compound() -> Result<()> {
        cycle(Nbt::make_compound(vec![(
            "list1".to_string(),
            Nbt::List {
                item_type: NbtType::Compound,
                children: vec![
                    Nbt::make_compound(vec![("int1".to_string(), Nbt::Int(43563456))]),
                    Nbt::make_compound(vec![("int2".to_string(), Nbt::Int(43563456))]),
                ],
            },
        )]))
    }
}
