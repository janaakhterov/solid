use crate::{
    bytes::Bytes,
    encode::Encode,
    Error,
    Result,
};
use serde::{
    ser,
    Serialize,
};
use std::{
    collections::VecDeque,
    convert::TryInto,
};

#[derive(Default)]
pub struct Serializer {
    sig: String,
    stack: VecDeque<Vec<Field>>,
}

#[derive(Default, Debug)]
struct Field {
    dynamic: bool,
    value: [u8; 32],
    buf: Vec<u8>,
}

impl Serializer {
    fn encode<T: Encode + IntoType>(&mut self, value: T) -> Result<(), Error> {
        if let Some(stack) = self.stack.front_mut() {
            let value = value.encode();
            if T::is_dynamic() {
                stack.push(Field {
                    dynamic: true,
                    value: [0u8; 32],
                    buf: value,
                });
            } else {
                stack.push(Field {
                    dynamic: false,
                    value: (value[..]).try_into()?,
                    buf: Vec::new(),
                });
            }
        }

        Ok(())
    }
}

pub fn to_bytes<T: ?Sized + Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut serializer = Serializer::default();
    value.serialize(&mut serializer)?;

    let mut stack = serializer.stack.pop_front().unwrap();

    let mut offset = stack.len() as u64 * 32;

    for field in stack.iter_mut() {
        if field.dynamic {
            field.value = (offset.encode()[..]).try_into()?;
            offset += field.buf.len() as u64;
        }
    }

    let buf = stack.iter().fold(Vec::new(), |mut buf, field| {
        buf.extend(&field.value);
        buf
    });

    let buf = stack.into_iter().fold(buf, |mut buf, field| {
        buf.extend(field.buf);
        buf
    });

    Ok(buf)
}

impl<'a> ser::Serializer for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    type SerializeSeq = Self;
    type SerializeTuple = Self;
    type SerializeTupleStruct = Self;
    type SerializeTupleVariant = Self;
    type SerializeMap = Self;
    type SerializeStruct = Self;
    type SerializeStructVariant = Self;

    fn serialize_bool(self, value: bool) -> Result<()> {
        self.encode(value)?;
        Ok(())
    }

    fn serialize_i8(self, value: i8) -> Result<()> {
        self.encode(value)?;
        Ok(())
    }

    fn serialize_u8(self, value: u8) -> Result<()> {
        self.encode(value)?;
        Ok(())
    }

    fn serialize_i16(self, value: i16) -> Result<()> {
        self.encode(value)?;
        Ok(())
    }

    fn serialize_u16(self, value: u16) -> Result<()> {
        self.encode(value)?;
        Ok(())
    }

    fn serialize_i32(self, value: i32) -> Result<()> {
        self.encode(value)?;
        Ok(())
    }

    fn serialize_u32(self, value: u32) -> Result<()> {
        self.encode(value)?;
        Ok(())
    }

    fn serialize_i64(self, value: i64) -> Result<()> {
        self.encode(value)?;
        Ok(())
    }

    fn serialize_u64(self, value: u64) -> Result<()> {
        self.encode(value)?;
        Ok(())
    }

    fn serialize_i128(self, value: i128) -> Result<()> {
        self.encode(value)?;
        Ok(())
    }

    fn serialize_u128(self, value: u128) -> Result<()> {
        self.encode(value)?;
        Ok(())
    }

    fn serialize_f32(self, _value: f32) -> Result<()> {
        Err(Error::Message(
            "Solidity does not support floats".to_string(),
        ))
    }

    fn serialize_f64(self, _value: f64) -> Result<()> {
        Err(Error::Message(
            "Solidity does not support floats".to_string(),
        ))
    }

    fn serialize_char(self, _value: char) -> Result<()> {
        Err(Error::Message(
            "Solidity does nnot support 'char' as a type".to_string(),
        ))
    }

    fn serialize_str(self, value: &str) -> Result<()> {
        self.encode(value)?;
        Ok(())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<()> {
        self.encode(Bytes(&value))?;
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        Err(Error::Message(
            "Solidity does not support optionals".to_string(),
        ))
    }

    fn serialize_some<T: ?Sized + Serialize>(self, _value: &T) -> Result<()> {
        Err(Error::Message(
            "Solidity does not support optionals".to_string(),
        ))
    }

    fn serialize_unit(self) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_struct(self, _name: &'static str) -> Result<()> {
        Ok(())
    }

    fn serialize_unit_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
    ) -> Result<()> {
        Err(Error::Message(
            "Solidity does not support enums".to_string(),
        ))
    }

    fn serialize_newtype_struct<T: ?Sized + Serialize>(
        self,
        _name: &'static str,
        value: &T,
    ) -> Result<()> {
        value.serialize(self)
    }

    fn serialize_newtype_variant<T>(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _value: &T,
    ) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
        Err(Error::Message(
            "Solidity does not support enums".to_string(),
        ))
    }

    fn serialize_seq(self, _len: Option<usize>) -> Result<Self::SerializeSeq> {
        self.stack.push_front(Vec::new());
        Ok(self)
    }

    fn serialize_tuple(self, len: usize) -> Result<Self::SerializeTuple> {
        self.serialize_struct("", len)
    }

    fn serialize_tuple_struct(
        self,
        name: &'static str,
        len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        self.serialize_struct(name, len)
    }

    fn serialize_tuple_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleVariant> {
        Err(Error::Message(
            "Solidity does not support enums".to_string(),
        ))
    }

    fn serialize_map(self, _len: Option<usize>) -> Result<Self::SerializeMap> {
        Err(Error::Message("Solidity does not support maps".to_string()))
    }

    fn serialize_struct(self, _name: &'static str, len: usize) -> Result<Self::SerializeStruct> {
        self.stack.push_front(Vec::with_capacity(len));
        Ok(self)
    }

    fn serialize_struct_variant(
        self,
        _name: &'static str,
        _variant_index: u32,
        _variant: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeStructVariant> {
        Err(Error::Message(
            "Solidity does not support enums".to_string(),
        ))
    }
}

impl<'a> ser::SerializeSeq for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut **self)?;
        Ok(())
    }

    fn end(self) -> Result<()> {
        if let Some(stack) = self.stack.front_mut() {
            let mut offset = stack.len() as u64 * 32;

            for field in stack.iter_mut() {
                if field.dynamic {
                    field.value = (offset.encode()[..]).try_into()?;
                    offset += field.buf.len() as u64;
                }
            }
        }

        if self.stack.len() >= 2 {
            let value_stack = self.stack.pop_front().unwrap();
            let stack = self.stack.front_mut().unwrap();

            let buf = (value_stack.len() as u64).encode();

            let buf = value_stack.iter().fold(buf, |mut buf, field| {
                buf.extend(&field.value);
                buf
            });

            let buf = value_stack.into_iter().fold(buf, |mut buf, field| {
                buf.extend(field.buf);
                buf
            });

            stack.push(Field {
                dynamic: true,
                value: [0u8; 32],
                buf,
            });
        }

        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        if let Some(stack) = self.stack.front_mut() {
            let mut offset = stack.len() as u64 * 32;

            for field in stack.iter_mut() {
                if field.dynamic {
                    field.value = (offset.encode()[..]).try_into()?;
                    offset += field.buf.len() as u64;
                }
            }
        }

        if self.stack.len() > 1 {
            let value_stack = self.stack.pop_front().unwrap();
            let stack = self.stack.front_mut().unwrap();

            let buf = value_stack.iter().fold(Vec::new(), |mut buf, field| {
                buf.extend(&field.value);
                buf
            });

            let buf = value_stack.into_iter().fold(buf, |mut buf, field| {
                buf.extend(field.buf);
                buf
            });

            stack.push(Field {
                dynamic: true,
                value: [0u8; 32],
                buf,
            });
        }

        Ok(())
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        if let Some(stack) = self.stack.front_mut() {
            let mut offset = stack.len() as u64 * 32;

            for field in stack.iter_mut() {
                if field.dynamic {
                    field.value = (offset.encode()[..]).try_into()?;
                    offset += field.buf.len() as u64;
                }
            }
        }

        if self.stack.len() > 1 {
            let value_stack = self.stack.pop_front().unwrap();
            let stack = self.stack.front_mut().unwrap();

            let buf = value_stack.iter().fold(Vec::new(), |mut buf, field| {
                buf.extend(&field.value);
                buf
            });

            let buf = value_stack.into_iter().fold(buf, |mut buf, field| {
                buf.extend(field.buf);
                buf
            });

            stack.push(Field {
                dynamic: true,
                value: [0u8; 32],
                buf,
            });
        }

        Ok(())
    }
}

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        if let Some(stack) = self.stack.front_mut() {
            let mut offset = stack.len() as u64 * 32;

            for field in stack.iter_mut() {
                if field.dynamic {
                    field.value = (offset.encode()[..]).try_into()?;
                    offset += field.buf.len() as u64;
                }
            }
        }

        if self.stack.len() > 1 {
            let value_stack = self.stack.pop_front().unwrap();
            let stack = self.stack.front_mut().unwrap();

            let buf = value_stack.iter().fold(Vec::new(), |mut buf, field| {
                buf.extend(&field.value);
                buf
            });

            let buf = value_stack.into_iter().fold(buf, |mut buf, field| {
                buf.extend(field.buf);
                buf
            });

            stack.push(Field {
                dynamic: true,
                value: [0u8; 32],
                buf,
            });
        }

        Ok(())
    }
}

impl<'a> ser::SerializeTupleVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<()> {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeMap for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_key<T: ?Sized + Serialize>(&mut self, _key: &T) -> Result<()> {
        unimplemented!()
    }

    fn serialize_value<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<()> {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeStructVariant for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _key: &'static str,
        _value: &T,
    ) -> Result<()> {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn serialize_numbers_test() -> Result<(), Error> {
        #[derive(Serialize)]
        struct Params {
            value0: i8,
            value1: u8,
            value2: i16,
            value3: u16,
            value4: i32,
            value5: u32,
            value6: i64,
            value7: u64,
            value8: i128,
            value9: u128,
        }

        let params = Params {
            value0: 0xffu8 as i8,
            value1: 0xffu8,
            value2: 0xffffu16 as i16,
            value3: 0xffffu16,
            value4: 0xffffffffu32 as i32,
            value5: 0xffffffffu32,
            value6: 0xffffffffffffffffu64 as i64,
            value7: 0xffffffffffffffffu64,
            value8: 0xffffffffffffffffffffffffffffffffu128 as i128,
            value9: 0xffffffffffffffffffffffffffffffffu128,
        };

        let buf = to_bytes(&params)?;

        let first = hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")
            .unwrap();
        let second =
            hex::decode("00000000000000000000000000000000000000000000000000000000000000ff")
                .unwrap();
        let third = hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")
            .unwrap();
        let forth = hex::decode("000000000000000000000000000000000000000000000000000000000000ffff")
            .unwrap();

        let fifth = hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")
            .unwrap();
        let sixth = hex::decode("00000000000000000000000000000000000000000000000000000000ffffffff")
            .unwrap();

        let seventh =
            hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")
                .unwrap();
        let eighth =
            hex::decode("000000000000000000000000000000000000000000000000ffffffffffffffff")
                .unwrap();

        let nineth =
            hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")
                .unwrap();
        let tenth = hex::decode("00000000000000000000000000000000ffffffffffffffffffffffffffffffff")
            .unwrap();

        assert_eq!(10 * 32, buf.len());
        assert_eq!(&first[0..32], &buf[32 * 0..32 * 1]);
        assert_eq!(&second[0..32], &buf[32 * 1..32 * 2]);
        assert_eq!(&third[0..32], &buf[32 * 2..32 * 3]);
        assert_eq!(&forth[0..32], &buf[32 * 3..32 * 4]);
        assert_eq!(&fifth[0..32], &buf[32 * 4..32 * 5]);
        assert_eq!(&sixth[0..32], &buf[32 * 5..32 * 6]);
        assert_eq!(&seventh[0..32], &buf[32 * 6..32 * 7]);
        assert_eq!(&eighth[0..32], &buf[32 * 7..32 * 8]);
        assert_eq!(&nineth[0..32], &buf[32 * 8..32 * 9]);
        assert_eq!(&tenth[0..32], &buf[32 * 9..32 * 10]);

        Ok(())
    }

    #[test]
    fn serialize_dynamic_test() -> Result<(), Error> {
        #[derive(Serialize)]
        struct Params<'a> {
            string: String,
            bytes: Bytes<'a>,
        }

        let bytes = Bytes(&b"random string"[..]);

        let params = Params {
            string: "random string".to_string(),
            bytes,
        };

        let buf = to_bytes(&params)?;

        let string_offset =
            hex::decode("0000000000000000000000000000000000000000000000000000000000000040")
                .unwrap();
        let bytes_offset =
            hex::decode("0000000000000000000000000000000000000000000000000000000000000080")
                .unwrap();
        let string_len =
            hex::decode("000000000000000000000000000000000000000000000000000000000000000D")
                .unwrap();
        let string =
            hex::decode("72616e646f6d20737472696e6700000000000000000000000000000000000000")
                .unwrap();
        let bytes_len =
            hex::decode("000000000000000000000000000000000000000000000000000000000000000D")
                .unwrap();
        let bytes = hex::decode("72616e646f6d20737472696e6700000000000000000000000000000000000000")
            .unwrap();

        assert_eq!(6 * 32, buf.len());
        assert_eq!(&string_offset[0..32], &buf[32 * 0..32 * 1]);
        assert_eq!(&bytes_offset[0..32], &buf[32 * 1..32 * 2]);
        assert_eq!(&string_len[0..32], &buf[32 * 2..32 * 3]);
        assert_eq!(&string[0..32], &buf[32 * 3..32 * 4]);
        assert_eq!(&bytes_len[0..32], &buf[32 * 4..32 * 5]);
        assert_eq!(&bytes[0..32], &buf[32 * 5..32 * 6]);

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_number_array() -> Result<(), Error> {
        #[derive(Serialize)]
        struct Params {
            first: u8,
            u64s: Vec<u64>,
            second: u8,
            i32s: Vec<i32>,
        }

        let params = Params {
            first: 0x44u8,
            u64s: vec![0xaau64, 0xbbu64],
            second: 0x55u8,
            i32s: vec![0xcci32, 0xddi32],
        };

        let buf = to_bytes(&params)?;

        let first       = hex::decode("0000000000000000000000000000000000000000000000000000000000000044") .unwrap();
        let u64s_offset = hex::decode("0000000000000000000000000000000000000000000000000000000000000080") .unwrap();
        let second      = hex::decode("0000000000000000000000000000000000000000000000000000000000000055") .unwrap();
        let i32s_offset = hex::decode("00000000000000000000000000000000000000000000000000000000000000e0") .unwrap();
        let u64s_len    = hex::decode("0000000000000000000000000000000000000000000000000000000000000002") .unwrap();
        let u64s_1      = hex::decode("00000000000000000000000000000000000000000000000000000000000000aa") .unwrap();
        let u64s_2      = hex::decode("00000000000000000000000000000000000000000000000000000000000000bb") .unwrap();
        let i32s_len    = hex::decode("0000000000000000000000000000000000000000000000000000000000000002") .unwrap();
        let i32s_1      = hex::decode("00000000000000000000000000000000000000000000000000000000000000cc") .unwrap();
        let i32s_2      = hex::decode("00000000000000000000000000000000000000000000000000000000000000dd") .unwrap();

        assert_eq!(10 * 32, buf.len());
        assert_eq!(&first[0..32],       &buf[32 * 0..32 * 1]);
        assert_eq!(&u64s_offset[0..32], &buf[32 * 1..32 * 2]);
        assert_eq!(&second[0..32],      &buf[32 * 2..32 * 3]);
        assert_eq!(&i32s_offset[0..32], &buf[32 * 3..32 * 4]);
        assert_eq!(&u64s_len[0..32],    &buf[32 * 4..32 * 5]);
        assert_eq!(&u64s_1[0..32],      &buf[32 * 5..32 * 6]);
        assert_eq!(&u64s_2[0..32],      &buf[32 * 6..32 * 7]);
        assert_eq!(&i32s_len[0..32],    &buf[32 * 7..32 * 8]);
        assert_eq!(&i32s_1[0..32],      &buf[32 * 8..32 * 9]);
        assert_eq!(&i32s_2[0..32],      &buf[32 * 9..32 * 10]);

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_tuple_test() -> Result<(), Error> {
        #[derive(Serialize)]
        struct Params<'a> {
            string: (String, &'a str),
            bytes: (Bytes<'a>, Bytes<'a>),
            numbers: (i8, u8, i16, u16, i32, u32),
        }

        let bytes1 = Bytes(&b"random string"[..]);
        let bytes2 = Bytes(&b"random string"[..]);

        let params = Params {
            string: ("random string".to_string(), "random string"),
            bytes: (bytes1, bytes2),
            numbers: (-2, 55, 1515, 8788, -151, 51515)
        };

        let buf = to_bytes(&params)?;

        let string_tuple_offset   = hex::decode("0000000000000000000000000000000000000000000000000000000000000060").unwrap();
        let bytes_tuple_offset    = hex::decode("0000000000000000000000000000000000000000000000000000000000000120").unwrap();
        let numbers_tuple_offset  = hex::decode("00000000000000000000000000000000000000000000000000000000000001e0").unwrap();
        let string_tuple_1_offset = hex::decode("0000000000000000000000000000000000000000000000000000000000000040").unwrap();
        let string_tuple_2_offset = hex::decode("0000000000000000000000000000000000000000000000000000000000000080").unwrap();
        let string1_len           = hex::decode("000000000000000000000000000000000000000000000000000000000000000D").unwrap();
        let string1               = hex::decode("72616e646f6d20737472696e6700000000000000000000000000000000000000").unwrap();
        let string2_len           = hex::decode("000000000000000000000000000000000000000000000000000000000000000D").unwrap();
        let string2               = hex::decode("72616e646f6d20737472696e6700000000000000000000000000000000000000").unwrap();
        let bytes_tuple_1_offset  = hex::decode("0000000000000000000000000000000000000000000000000000000000000040").unwrap();
        let bytes_tuple_2_offset  = hex::decode("0000000000000000000000000000000000000000000000000000000000000080").unwrap();
        let bytes1_len            = hex::decode("000000000000000000000000000000000000000000000000000000000000000D").unwrap();
        let bytes1                = hex::decode("72616e646f6d20737472696e6700000000000000000000000000000000000000").unwrap();
        let bytes2_len            = hex::decode("000000000000000000000000000000000000000000000000000000000000000D").unwrap();
        let bytes2                = hex::decode("72616e646f6d20737472696e6700000000000000000000000000000000000000").unwrap();
        let number_i8             = hex::decode("fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe").unwrap();
        let number_u8             = hex::decode("0000000000000000000000000000000000000000000000000000000000000037").unwrap();
        let number_i16            = hex::decode("00000000000000000000000000000000000000000000000000000000000005eb").unwrap();
        let number_u16            = hex::decode("0000000000000000000000000000000000000000000000000000000000002254").unwrap();
        let number_i32            = hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff69").unwrap();
        let number_u32            = hex::decode("000000000000000000000000000000000000000000000000000000000000c93b").unwrap();

        assert_eq!(&string_tuple_offset[0..32],   &buf[32 * 0..32 * 1]);
        assert_eq!(&bytes_tuple_offset[0..32],    &buf[32 * 1..32 * 2]);
        assert_eq!(&numbers_tuple_offset[0..32],  &buf[32 * 2..32 * 3]);
        assert_eq!(&string_tuple_1_offset[0..32], &buf[32 * 3..32 * 4]);
        assert_eq!(&string_tuple_2_offset[0..32], &buf[32 * 4..32 * 5]);
        assert_eq!(&string1_len[0..32],           &buf[32 * 5..32 * 6]);
        assert_eq!(&string1[0..32],               &buf[32 * 6..32 * 7]);
        assert_eq!(&string2_len[0..32],           &buf[32 * 7..32 * 8]);
        assert_eq!(&string2[0..32],               &buf[32 * 8..32 * 9]);
        assert_eq!(&bytes_tuple_1_offset[0..32],  &buf[32 * 9..32 * 10]);
        assert_eq!(&bytes_tuple_2_offset[0..32],  &buf[32 * 10..32 * 11]);
        assert_eq!(&bytes1_len[0..32],            &buf[32 * 11..32 * 12]);
        assert_eq!(&bytes1[0..32],                &buf[32 * 12..32 * 13]);
        assert_eq!(&bytes2_len[0..32],            &buf[32 * 13..32 * 14]);
        assert_eq!(&bytes2[0..32],                &buf[32 * 14..32 * 15]);
        assert_eq!(&number_i8[0..32],             &buf[32 * 15..32 * 16]);
        assert_eq!(&number_u8[0..32],             &buf[32 * 16..32 * 17]);
        assert_eq!(&number_i16[0..32],            &buf[32 * 17..32 * 18]);
        assert_eq!(&number_u16[0..32],            &buf[32 * 18..32 * 19]);
        assert_eq!(&number_i32[0..32],            &buf[32 * 19..32 * 20]);
        assert_eq!(&number_u32[0..32],            &buf[32 * 20..32 * 21]);

        Ok(())
    }
}
