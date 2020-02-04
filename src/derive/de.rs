use crate::from_bytes::FromBytes;
use crate::Error;
use crate::Result;
use serde::de::{self, DeserializeSeed, SeqAccess, Visitor};
use serde::Deserialize;
use std::mem;

pub struct Deserializer<'de> {
    buf: &'de [u8],
    index: usize,
}

impl<'de> Deserializer<'de> {
    pub fn from_bytes(buf: &'de [u8]) -> Self {
        Deserializer { buf, index: 0 }
    }
}

// By convention, the public API of a Serde deserializer is one or more
// `from_xyz` methods such as `from_str`, `from_bytes`, or `from_reader`
// depending on what Rust types the deserializer is able to consume as input.
//
// This basic deserializer supports only `from_str`.
pub fn from_bytes<'a, T>(buf: &'a [u8]) -> Result<T>
where
    T: Deserialize<'a>,
{
    let mut deserializer = Deserializer::from_bytes(buf);
    Ok(T::deserialize(&mut deserializer)?)
}

impl<'de> Deserializer<'de> {
    fn next_block(&mut self) -> Result<&'de [u8]> {
        if let (Some(_), Some(_)) = (
            self.buf.get(self.index * 32),
            self.buf.get((self.index + 1) * 32 - 1),
        ) {
            let buf = &self.buf[self.index * 32..(self.index + 1) * 32];
            self.index += 1;
            Ok(&buf)
        } else {
            Err(Error::Eof)
        }
    }

    // Parse the JSON identifier `true` or `false`.
    fn parse_bool(&mut self) -> Result<bool> {
        Ok(self.next_block()?[31] == 1)
    }

    fn parse_number<T>(&mut self) -> Result<T>
    where
        T: FromBytes<'de, T>,
    {
        let block = self.next_block()?;
        T::from_bytes(&block[32 - mem::size_of::<T>()..32])
    }

    fn parse_string(&mut self) -> Result<&'de str> {
        Ok(std::str::from_utf8(&self.parse_bytes()?)?)
    }

    fn parse_bytes(&mut self) -> Result<&'de [u8]> {
        let offset = self.parse_number::<usize>()?;
        println!("[parse_bytes] offset: {}", offset);
        println!(
            "[parse_bytes] len: {}",
            usize::from_bytes(&self.buf[offset + 32 - mem::size_of::<usize>()..offset + 32])?
        );
        println!("[parse_bytes] buf: {:?}", &self.buf[..]);
        match (
            self.buf.get(offset),
            usize::from_bytes(&self.buf[offset + 32 - mem::size_of::<usize>()..offset + 32])?,
        ) {
            (Some(_), len) if self.buf.get(offset + len).is_some() => {
                Ok(&self.buf[offset + 32..offset + 32 + len])
            }

            _ => Err(Error::Message(
                "Failed to parse string because buffer doesn't contain entire string".to_string(),
            )),
        }
    }
}

impl<'de, 'a> de::Deserializer<'de> for &'a mut Deserializer<'de> {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        unimplemented!()
    }

    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_bool(self.parse_bool()?)
    }

    fn deserialize_i8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i8(self.parse_number::<i8>()?)
    }

    fn deserialize_i16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i16(self.parse_number::<i16>()?)
    }

    fn deserialize_i32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i32(self.parse_number::<i32>()?)
    }

    fn deserialize_i64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i64(self.parse_number::<i64>()?)
    }

    fn deserialize_i128<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_i128(self.parse_number::<i128>()?)
    }

    fn deserialize_u8<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u8(self.parse_number::<u8>()?)
    }

    fn deserialize_u16<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u16(self.parse_number::<u16>()?)
    }

    fn deserialize_u32<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u32(self.parse_number::<u32>()?)
    }

    fn deserialize_u64<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u64(self.parse_number::<u64>()?)
    }

    fn deserialize_u128<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_u128(self.parse_number::<u128>()?)
    }

    // UNSUPPORTED
    fn deserialize_f32<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        unimplemented!()
    }

    // UNSUPPORTED
    fn deserialize_f64<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        unimplemented!()
    }

    // UNSUPPORTED
    fn deserialize_char<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        unimplemented!()
    }

    fn deserialize_str<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_borrowed_str(self.parse_string()?)
    }

    fn deserialize_string<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_str(visitor)
    }

    fn deserialize_byte_buf<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_borrowed_bytes(self.parse_bytes()?)
    }

    // The `Serializer` implementation on the previous page serialized byte
    // arrays as JSON arrays of bytes. Handle that representation here.
    fn deserialize_bytes<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        self.deserialize_byte_buf(visitor)
    }

    // An absent optional is represented as the JSON `null` and a present
    // optional is represented as just the contained value.
    //
    // As commented in `Serializer` implementation, this is a lossy
    // representation. For example the values `Some(())` and `None` both
    // serialize as just `null`. Unfortunately this is typically what people
    // expect when working with JSON. Other formats are encouraged to behave
    // more intelligently if possible.
    fn deserialize_option<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        unimplemented!()
    }

    // In Serde, unit means an anonymous value containing no data.
    fn deserialize_unit<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value> {
        visitor.visit_unit()
    }

    // Unit struct means a named value containing no data.
    fn deserialize_unit_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_unit(visitor)
    }

    // As is done here, serializers are encouraged to treat newtype structs as
    // insignificant wrappers around the data they contain. That means not
    // parsing anything other than the contained value.
    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value> {
        visitor.visit_newtype_struct(self)
    }

    // Deserialization of compound types like sequences and maps happens by
    // passing the visitor an "Access" object that gives it the ability to
    // iterate through the data contained in the sequence.
    fn deserialize_seq<V: Visitor<'de>>(mut self, visitor: V) -> Result<V::Value> {
        Ok(visitor.visit_seq(Struct::new(&mut self))?)
    }

    // Tuples look just like sequences in JSON. Some formats may be able to
    // represent tuples more efficiently.
    //
    // As indicated by the length parameter, the `Deserialize` implementation
    // for a tuple in the Serde data model is required to know the length of the
    // tuple before even looking at the input data.
    fn deserialize_tuple<V: Visitor<'de>>(self, _len: usize, _visitor: V) -> Result<V::Value> {
        // self.deserialize_seq(visitor)
        unimplemented!()
    }

    // Tuple structs look just like sequences in JSON.
    fn deserialize_tuple_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _len: usize,
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_seq(visitor)
    }

    // Much like `deserialize_seq` but calls the visitors `visit_map` method
    // with a `MapAccess` implementation, rather than the visitor's `visit_seq`
    // method with a `SeqAccess` implementation.
    fn deserialize_map<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        unimplemented!()
        // Ok(visitor.visit_map(Struct::new(&mut self))?)
    }

    // Structs look just like maps in JSON.
    //
    // Notice the `fields` parameter - a "struct" in the Serde data model means
    // that the `Deserialize` implementation is required to know what the fields
    // are before even looking at the input data. Any key-value pairing in which
    // the fields cannot be known ahead of time is probably a map.
    fn deserialize_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value> {
        self.deserialize_seq(visitor)
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        _visitor: V,
    ) -> Result<V::Value> {
        unimplemented!()
    }

    fn deserialize_identifier<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        Err(Error::Message(
            "Solidity does not support Deserializer::deserialize_identifier".to_string(),
        ))
    }

    fn deserialize_ignored_any<V: Visitor<'de>>(self, _visitor: V) -> Result<V::Value> {
        unimplemented!()
        // self.deserialize_any(visitor)
    }
}

struct Struct<'a, 'de> {
    de: &'a mut Deserializer<'de>,
}

impl<'a, 'de> Struct<'a, 'de> {
    fn new(de: &'a mut Deserializer<'de>) -> Self {
        Struct { de }
    }
}

impl<'de, 'a> SeqAccess<'de> for Struct<'a, 'de> {
    type Error = Error;

    fn next_element_seed<T: DeserializeSeed<'de>>(&mut self, seed: T) -> Result<Option<T::Value>> {
        seed.deserialize(&mut *self.de).map(Some)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    #[rustfmt::skip]
    fn de_number_test() -> Result<()> {
        #[derive(Debug, Deserialize)]
        struct Response {
            r#i8: i8,
            r#u8: u8,
            r#i16: i16,
            r#u16: u16,
            r#i32: i32,
            r#u32: u32,
            r#i64: i64,
            r#u64: u64,
            r#i128: i128,
            r#u128: u128,
        }

        let value = hex::decode(
            "\
00000000000000000000000000000000000000000000000000000000000000FF\
00000000000000000000000000000000000000000000000000000000000000FF\
000000000000000000000000000000000000000000000000000000000000FFFF\
000000000000000000000000000000000000000000000000000000000000FFFF\
00000000000000000000000000000000000000000000000000000000FFFFFFFF\
00000000000000000000000000000000000000000000000000000000FFFFFFFF\
000000000000000000000000000000000000000000000000FFFFFFFFFFFFFFFF\
000000000000000000000000000000000000000000000000FFFFFFFFFFFFFFFF\
00000000000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF\
00000000000000000000000000000000FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFF",
        )
        .unwrap();
        let value: Response = from_bytes(&value)?;

        assert_eq!(value.r#i8,   0xFFu8 as i8);
        assert_eq!(value.r#u8,   0xFFu8);
        assert_eq!(value.r#i16,  0xFFFFu16 as i16);
        assert_eq!(value.r#u16,  0xFFFFu16);
        assert_eq!(value.r#i32,  0xFFFFFFFFu32 as i32);
        assert_eq!(value.r#u32,  0xFFFFFFFFu32);
        assert_eq!(value.r#i64,  0xFFFFFFFFFFFFFFFFu64 as i64);
        assert_eq!(value.r#u64,  0xFFFFFFFFFFFFFFFFu64);
        assert_eq!(value.r#i128, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128 as i128);
        assert_eq!(value.r#u128, 0xFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFu128);

        Ok(())
    }

    #[test]
    #[rustfmt::skip]
    fn de_string_test() -> Result<()> {
        #[derive(Debug, Deserialize)]
        struct Response<'a> {
            string: &'a str,
            bytes: &'a [u8],
        }

        let value = hex::decode(
            "\
0000000000000000000000000000000000000000000000000000000000000040\
0000000000000000000000000000000000000000000000000000000000000080\
000000000000000000000000000000000000000000000000000000000000000C\
72616E646F6D2062797465730000000000000000000000000000000000000000\
000000000000000000000000000000000000000000000000000000000000000C\
72616E646F6D2062797465730000000000000000000000000000000000000000\
",
        )
        .unwrap();

        let value: Response = from_bytes(&value)?;

        assert_eq!(value.string.len(), 12);
        assert_eq!(value.string, "random bytes");

        assert_eq!(value.bytes.len(), 12);
        assert_eq!(value.bytes, &b"random bytes"[..]);

        Ok(())
    }

    // #[test]
    // #[rustfmt::skip]
    // fn de_string_array_test() -> Result<()> {
    //     #[derive(Debug, Deserialize)]
    //     struct Response {
    //         strings: Vec<String>,
    //     }

    //     let value = hex::decode(
    //         "\
    // 0000000000000000000000000000000000000000000000000000000000000020\
    // 0000000000000000000000000000000000000000000000000000000000000002\
    // 0000000000000000000000000000000000000000000000000000000000000040\
    // 0000000000000000000000000000000000000000000000000000000000000080\
    // 000000000000000000000000000000000000000000000000000000000000000C\
    // 72616E646F6D2062797465730000000000000000000000000000000000000000\
    // 000000000000000000000000000000000000000000000000000000000000000C\
    // 72616E646F6D2062797465730000000000000000000000000000000000000000\
    // ",
    //     )
    //     .unwrap();

    //     let value: Response = from_bytes(&value)?;

    //     assert_eq!(value.strings.len(), 2);
    //     // assert_eq!(value.string, "random bytes");

    //     Ok(())
    // }
}
