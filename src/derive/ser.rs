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
use std::collections::VecDeque;

#[derive(Default)]
pub struct Serializer {
    stack: VecDeque<Buf>,
}

#[derive(Default, Debug)]
struct Buf {
    statics: Vec<u8>,
    dynamics: Vec<u8>,
}

impl Serializer {
    fn encode<T: Encode>(&mut self, value: T) {
        if let Some(stack) = self.stack.front_mut() {
            println!("[Serializer::Encode], dynamic: {}", T::is_dynamic());
            let value = value.encode();
            if T::is_dynamic() {
                stack.dynamics.extend_from_slice(&value);
                stack
                    .statics
                    .extend_from_slice(&(stack.dynamics.len() as u64).encode());
            } else {
                stack.statics.extend_from_slice(&value);
            }
        }
    }
}

pub fn to_bytes<T: ?Sized + Serialize>(value: &T) -> Result<Vec<u8>> {
    let mut serializer = Serializer::default();

    println!("[to_bytes] start");
    println!(
        "[to_bytes] serializer stack.len(): {}",
        serializer.stack.len()
    );
    value.serialize(&mut serializer)?;
    println!("[to_bytes] serializer finished");
    println!(
        "[to_bytes] serializer stack.len(): {}",
        serializer.stack.len()
    );

    if let Some(mut stack) = serializer.stack.pop_front() {
        println!("[to_bytes] combining");
        stack.statics.extend_from_slice(&mut stack.dynamics);
        Ok(stack.statics)
    } else {
        println!("[to_bytes] new vec");
        Ok(Vec::new())
    }
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
        println!("[Serialize] bool");
        self.encode(value);
        Ok(())
    }

    fn serialize_i8(self, value: i8) -> Result<()> {
        println!("[Serialize] i8");
        self.encode(value);
        Ok(())
    }

    fn serialize_u8(self, value: u8) -> Result<()> {
        println!("[Serialize] u8");
        self.encode(value);
        Ok(())
    }

    fn serialize_i16(self, value: i16) -> Result<()> {
        println!("[Serialize] i16");
        self.encode(value);
        Ok(())
    }

    fn serialize_u16(self, value: u16) -> Result<()> {
        println!("[Serialize] u16");
        self.encode(value);
        Ok(())
    }

    fn serialize_i32(self, value: i32) -> Result<()> {
        println!("[Serialize] i32");
        self.encode(value);
        Ok(())
    }

    fn serialize_u32(self, value: u32) -> Result<()> {
        println!("[Serialize] u32");
        self.encode(value);
        Ok(())
    }

    fn serialize_i64(self, value: i64) -> Result<()> {
        println!("[Serialize] i64");
        self.encode(value);
        Ok(())
    }

    fn serialize_u64(self, value: u64) -> Result<()> {
        println!("[Serialize] u64");
        self.encode(value);
        Ok(())
    }

    fn serialize_i128(self, value: i128) -> Result<()> {
        println!("[Serialize] i128");
        self.encode(value);
        Ok(())
    }

    fn serialize_u128(self, value: u128) -> Result<()> {
        println!("[Serialize] u128");
        self.encode(value);
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
        return Err(Error::Message(
            "Solidity does nnot support 'char' as a type".to_string(),
        ));
    }

    fn serialize_str(self, value: &str) -> Result<()> {
        println!("[Serialize] str");
        self.encode(value);
        Ok(())
    }

    fn serialize_bytes(self, value: &[u8]) -> Result<()> {
        println!("[Serialize] bytes");
        self.encode(Bytes(&value));
        Ok(())
    }

    fn serialize_none(self) -> Result<()> {
        Err(Error::Message(
            "Solidity does not support optionals".to_string(),
        ))
    }

    fn serialize_some<T>(self, _value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
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

    fn serialize_newtype_struct<T>(self, _name: &'static str, value: &T) -> Result<()>
    where
        T: ?Sized + Serialize,
    {
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
        self.stack.push_front(Buf::default());
        Ok(self)
    }

    fn serialize_tuple(self, _len: usize) -> Result<Self::SerializeTuple> {
        println!("[Serialize] tuple");
        self.stack.push_front(Buf::default());
        Ok(self)
    }

    fn serialize_tuple_struct(
        self,
        _name: &'static str,
        _len: usize,
    ) -> Result<Self::SerializeTupleStruct> {
        println!("[Serialize] tuple_struct");
        self.stack.push_front(Buf::default());
        Ok(self)
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

    fn serialize_struct(self, _name: &'static str, _len: usize) -> Result<Self::SerializeStruct> {
        println!("[Serialize] struct");
        self.stack.push_front(Buf::default());
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
        let mut value_stack = self.stack.pop_front().unwrap();
        let mut stack = self.stack.remove(0).unwrap();
        stack
            .statics
            .extend_from_slice(&(value_stack.statics.len() as u64 / 32).encode());
        value_stack.statics.extend_from_slice(&value_stack.dynamics);
        stack.dynamics.extend_from_slice(&value_stack.statics);
        Ok(())
    }
}

impl<'a> ser::SerializeTuple for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_element<T: ?Sized + Serialize>(&mut self, _value: &T) -> Result<()> {
        unimplemented!()
    }

    fn end(self) -> Result<()> {
        unimplemented!()
    }
}

impl<'a> ser::SerializeTupleStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(&mut self, value: &T) -> Result<()> {
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        let mut value_stack = self.stack.pop_front().unwrap();
        let mut stack = self.stack.remove(0).unwrap();
        stack
            .statics
            .extend_from_slice(&(value_stack.statics.len() as u64 / 32).encode());
        value_stack.statics.extend_from_slice(&value_stack.dynamics);
        stack.dynamics.extend_from_slice(&value_stack.statics);
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

impl<'a> ser::SerializeStruct for &'a mut Serializer {
    type Ok = ();
    type Error = Error;

    fn serialize_field<T: ?Sized + Serialize>(
        &mut self,
        _key: &'static str,
        value: &T,
    ) -> Result<()> {
        println!("[SerializeStruct] field");
        value.serialize(&mut **self)
    }

    fn end(self) -> Result<()> {
        println!("[SerializeStruct] end");
        println!("[SerializeStruct] stack.len(): {}", self.stack.len());
        if self.stack.len() > 2 {
            if let (Some(mut value_stack), Some(stack)) =
                (self.stack.pop_front(), self.stack.front_mut())
            {
                println!("[SerializeStruct] value_stack: {:?}", value_stack);
                println!("[SerializeStruct] stack: {:?}", stack);

                stack
                    .statics
                    .extend_from_slice(&(value_stack.statics.len() as u64 / 32).encode());
                value_stack.statics.extend_from_slice(&value_stack.dynamics);
                stack.dynamics.extend_from_slice(&value_stack.statics);
            }
        }
        println!("[Serialize] struct_end finished");
        Ok(())
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

        println!("{:?}", buf);

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

        // let bytes = &b"random string"[..];
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
}
