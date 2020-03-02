use crate::nbt::Nbt;
use crate::result::*;
use bytes::buf::Buf;
use bytes::buf::BufMut;
use bytes::BytesMut;
use enum_primitive::FromPrimitive;
use uuid::Uuid;

pub fn get_var_int_len(value: i32) -> usize {
    let mut value = value as u32;
    let mut i = 1;
    while (value & !0b1111111) != 0 {
        i += 1;
        value >>= 7;
        if i > 5 {
            break;
        }
    }
    return i;
}

pub fn invalid_data<T>() -> Result<T> {
    return Err(Box::new(IoError::from(ErrorKind::InvalidData)));
}

pub trait McProtoBase {

    fn get_mc_var_int(&mut self) -> Result<i32>;
    fn get_mc_var_long(&mut self) -> Result<i64>;
    fn get_mc_string(&mut self, bound: i32) -> Result<String>;
    fn get_mc_u8(&mut self) -> Result<u8>;
    fn get_mc_i8(&mut self) -> Result<i8>;
    fn get_mc_bool(&mut self) -> Result<bool>;
    fn get_mc_i16(&mut self) -> Result<i16>;
    fn get_mc_i64(&mut self) -> Result<i64>;
    fn get_mc_f64(&mut self) -> Result<f64>;
    fn get_mc_f32(&mut self) -> Result<f32>;
    fn get_mc_uuid(&mut self) -> Result<Uuid>;
    fn get_mc_i32(&mut self) -> Result<i32>;
    fn get_mc_nbt(&mut self) -> Result<Nbt>;
    fn get_mc_u16(&mut self) -> Result<u16>;
    fn get_mc_byte_array(&mut self) -> Result<Vec<u8>>;
    fn get_mc_byte_array_bounded(&mut self, bound: i32) -> Result<Vec<u8>>;
    fn get_mc_enum<T: FromPrimitive>(&mut self) -> Result<T>;
    fn get_mc_enum_i32<T: FromPrimitive>(&mut self) -> Result<T>;
    fn get_mc_enum_u8<T: FromPrimitive>(&mut self) -> Result<T>;
    fn set_mc_var_int(&mut self, value: i32);
    fn set_mc_var_long(&mut self, value: i64);
    fn set_mc_string(&mut self, value: String);
    fn set_mc_u8(&mut self, value: u8);
    fn set_mc_i8(&mut self, value: i8);
    fn set_mc_bool(&mut self, value: bool);
    fn set_mc_i16(&mut self, value: i16);
    fn set_mc_i64(&mut self, value: i64);
    fn set_mc_f64(&mut self, value: f64);
    fn set_mc_f32(&mut self, value: f32);
    fn set_mc_uuid(&mut self, value: Uuid);
    fn set_mc_i32(&mut self, value: i32);
    fn set_mc_nbt(&mut self, value: Nbt);
    fn set_mc_u16(&mut self, value: u16);
    fn set_mc_byte_array(&mut self, value: Vec<u8>);

    fn clone_bounded(&mut self, bound: i32) -> Result<Self>
    where
        Self: Sized;

    fn read_primitive_slice<T: Sized + Clone>(&mut self, length: usize) -> Result<Vec<T>>;
    fn write_primitive_slice<T: Sized>(&mut self, data: &[T]);
    fn display(&self) -> String;
}

impl McProtoBase for BytesMut {
    fn get_mc_var_int(&mut self) -> Result<i32> {
        let mut i = 0;
        let mut current_byte: u8 = 0xff;
        let mut output: i32 = 0;
        while (current_byte & 128) == 128 {
            current_byte = self.get_mc_u8()?;
            output |= ((current_byte as u32 & 127) << (i * 7)) as i32;
            i += 1;
            if i > 5 {
                break;
            }
        }
        Ok(output)
    }

    fn get_mc_var_long(&mut self) -> Result<i64> {
        let mut i = 0;
        let mut current_byte: u8 = 0xff;
        let mut output: i64 = 0;
        while (current_byte & 128) == 128 {
            current_byte = self.get_mc_u8()?;
            output |= ((current_byte as u64 & 127) << (i * 7)) as i64;
            i += 1;
            if i > 10 {
                break;
            }
        }
        Ok(output)
    }

    fn get_mc_string(&mut self, bound: i32) -> Result<String> {
        let length = self.get_mc_var_int()?;
        if length > bound * 4 || length < 0 || length as usize > self.len() {
            invalid_data()
        } else {
            let string_value = String::from_utf8(self.split_to(length as usize).to_vec())?;
            if string_value.len() > bound as usize {
                invalid_data()
            } else {
                Ok(string_value)
            }
        }
    }

    fn get_mc_u8(&mut self) -> Result<u8> {
        if self.len() < 1 {
            invalid_data()
        } else {
            Ok(self.get_u8())
        }
    }

    fn get_mc_i8(&mut self) -> Result<i8> {
        if self.len() < 1 {
            invalid_data()
        } else {
            Ok(self.get_i8())
        }
    }

    fn get_mc_bool(&mut self) -> Result<bool> {
        if self.len() < 1 {
            invalid_data()
        } else {
            Ok(self.get_u8() != 0)
        }
    }

    fn get_mc_i16(&mut self) -> Result<i16> {
        if self.len() < 2 {
            invalid_data()
        } else {
            Ok(self.get_i16())
        }
    }

    fn get_mc_i64(&mut self) -> Result<i64> {
        if self.len() < 8 {
            invalid_data()
        } else {
            Ok(self.get_i64())
        }
    }

    fn get_mc_f64(&mut self) -> Result<f64> {
        if self.len() < 8 {
            invalid_data()
        } else {
            Ok(self.get_f64())
        }
    }

    fn get_mc_f32(&mut self) -> Result<f32> {
        if self.len() < 4 {
            invalid_data()
        } else {
            Ok(self.get_f32())
        }
    }

    fn get_mc_uuid(&mut self) -> Result<Uuid> {
        if self.len() < 16 {
            invalid_data()
        } else {
            Ok(Uuid::from_bytes(&self.split_to(16).to_vec())?)
        }
    }

    fn get_mc_i32(&mut self) -> Result<i32> {
        if self.len() < 4 {
            invalid_data()
        } else {
            Ok(self.get_i32())
        }
    }

    fn get_mc_nbt(&mut self) -> Result<Nbt> {
        Ok(Nbt::parse(self)?)
    }

    fn get_mc_u16(&mut self) -> Result<u16> {
        if self.len() < 2 {
            invalid_data()
        } else {
            Ok(self.get_u16())
        }
    }

    fn get_mc_byte_array(&mut self) -> Result<Vec<u8>> {
        Ok(self.to_vec())
    }

    fn get_mc_byte_array_bounded(&mut self, bound: i32) -> Result<Vec<u8>> {
        let length = self.get_mc_var_int()?;
        if length > bound || length < 0 || length as usize > self.len() {
            return invalid_data();
        }
        Ok(self.split_to(length as usize).to_vec())
    }

    fn get_mc_enum<T: FromPrimitive>(&mut self) -> Result<T> {
        let action = T::from_i32(self.get_mc_var_int()?);
        let action = match action {
            Some(action) => action,
            None => {
                return invalid_data();
            }
        };
        return Ok(action);
    }

    fn get_mc_enum_i32<T: FromPrimitive>(&mut self) -> Result<T> {
        let action = T::from_i32(self.get_mc_i32()?);
        let action = match action {
            Some(action) => action,
            None => {
                return invalid_data();
            }
        };
        return Ok(action);
    }

    fn get_mc_enum_u8<T: FromPrimitive>(&mut self) -> Result<T> {
        let action = T::from_u8(self.get_mc_u8()?);
        let action = match action {
            Some(action) => action,
            None => {
                return invalid_data();
            }
        };
        return Ok(action);
    }

    fn set_mc_var_int(&mut self, value: i32) {
        let mut value = value as u32;
        self.reserve(6);
        while (value & !0b1111111) != 0 {
            self.put_u8((value as u8 & 127) | 128);
            value >>= 7;
        }
        self.put_u8(value as u8);
    }

    fn set_mc_var_long(&mut self, value: i64) {
        let mut value = value as u64;
        self.reserve(12);
        while (value & !0b1111111) != 0 {
            self.put_u8((value as u8 & 127) | 128);
            value >>= 7;
        }
        self.put_u8(value as u8);
    }

    fn set_mc_string(&mut self, value: String) {
        let bytes = value.as_bytes();
        self.set_mc_var_int(bytes.len() as i32);
        self.extend(bytes);
    }

    fn set_mc_u8(&mut self, value: u8) {
        self.reserve(1);
        self.put_u8(value);
    }

    fn set_mc_i8(&mut self, value: i8) {
        self.reserve(1);
        self.put_i8(value);
    }

    fn set_mc_bool(&mut self, value: bool) {
        self.reserve(1);
        self.put_u8(if value { 1 } else { 0 });
    }

    fn set_mc_i16(&mut self, value: i16) {
        self.reserve(2);
        self.put_i16(value);
    }

    fn set_mc_i64(&mut self, value: i64) {
        self.reserve(8);
        self.put_i64(value);
    }

    fn set_mc_f64(&mut self, value: f64) {
        self.reserve(8);
        self.put_f64(value);
    }

    fn set_mc_f32(&mut self, value: f32) {
        self.reserve(4);
        self.put_f32(value);
    }

    fn set_mc_uuid(&mut self, value: Uuid) {
        self.reserve(16);
        self.extend_from_slice(value.as_bytes());
    }

    fn set_mc_i32(&mut self, value: i32) {
        self.reserve(4);
        self.put_i32(value);
    }

    fn set_mc_nbt(&mut self, value: Nbt) {
        value.serialize(self);
    }

    fn set_mc_u16(&mut self, value: u16) {
        self.reserve(2);
        self.put_u16(value);
    }

    fn set_mc_byte_array(&mut self, value: Vec<u8>) {
        self.set_mc_var_int(value.len() as i32);
        self.extend(value);
    }

    fn clone_bounded(&mut self, bound: i32) -> Result<BytesMut> {
        if self.len() > bound as usize {
            return invalid_data();
        }
        let returned = self.clone();
        let advanced = self.len();
        self.advance(advanced);
        Ok(returned)
    }

    fn read_primitive_slice<T: Sized + Clone>(&mut self, length: usize) -> Result<Vec<T>> {
        let raw_length = length * std::mem::size_of::<T>();
        if self.len() < raw_length {
            return invalid_data();
        }
        let raw = &self.split_to(raw_length)[..];
        Ok(unsafe { std::slice::from_raw_parts(raw.as_ptr() as *const T, length) }.to_vec())
    }

    fn write_primitive_slice<T: Sized>(&mut self, data: &[T]) {
        let raw = unsafe {
            std::slice::from_raw_parts(
                data.as_ptr() as *const u8,
                data.len() * std::mem::size_of::<T>(),
            )
        };
        self.extend_from_slice(raw);
    }

    fn display(&self) -> String {
        return format!("{:x?}", &self.to_vec()[..]);
    }
}