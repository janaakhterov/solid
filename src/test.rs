#[cfg(test)]
use crate::{
    builder::Builder,
    bytes::Bytes,
    bytesfix::Bytes4,
    address::Address,
    function::Function,
};
#[cfg(test)]
use std::convert::TryFrom;

#[test]
#[rustfmt::skip]
fn number_test() {
    let buf = Builder::new()
        .add(0xffu8 as i8)
        .add(0xffu8)
        .add(0xffffu16 as i16)
        .add(0xffffu16)
        .add(0xffffffffu32 as i32)
        .add(0xffffffffu32)
        .add(0xffffffffffffffffu64 as i64)
        .add(0xffffffffffffffffu64)
        .add(0xffffffffffffffffffffffffffffffffu128 as i128)
        .add(0xffffffffffffffffffffffffffffffffu128)
        .build();
    let first = hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
    let second = hex::decode("00000000000000000000000000000000000000000000000000000000000000ff").unwrap();
    let third = hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
    let forth = hex::decode("000000000000000000000000000000000000000000000000000000000000ffff").unwrap();
    let fifth = hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
    let sixth = hex::decode("00000000000000000000000000000000000000000000000000000000ffffffff").unwrap();
    let seventh = hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
    let eighth = hex::decode("000000000000000000000000000000000000000000000000ffffffffffffffff").unwrap();
    let nineth = hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
    let tenth = hex::decode("00000000000000000000000000000000ffffffffffffffffffffffffffffffff").unwrap();

    assert_eq!(&first[0..32],   &buf[32 * 0..32 * 1]);
    assert_eq!(&second[0..32],  &buf[32 * 1..32 * 2]);
    assert_eq!(&third[0..32],   &buf[32 * 2..32 * 3]);
    assert_eq!(&forth[0..32],   &buf[32 * 3..32 * 4]);
    assert_eq!(&fifth[0..32],   &buf[32 * 4..32 * 5]);
    assert_eq!(&sixth[0..32],   &buf[32 * 5..32 * 6]);
    assert_eq!(&seventh[0..32], &buf[32 * 6..32 * 7]);
    assert_eq!(&eighth[0..32],  &buf[32 * 7..32 * 8]);
    assert_eq!(&nineth[0..32],  &buf[32 * 8..32 * 9]);
    assert_eq!(&tenth[0..32],   &buf[32 * 9..32 * 10]);
}

#[test]
#[rustfmt::skip]
fn byte_test() {
    let buf = Builder::new().add(Bytes("random bytes".as_bytes())).build();
    let first_offset = hex::decode("0000000000000000000000000000000000000000000000000000000000000020").unwrap();
    let first_len = hex::decode("000000000000000000000000000000000000000000000000000000000000000c").unwrap();
    let first_data = hex::decode("72616E646F6D206279746573000000000000000000000000000000000000000000").unwrap();

    assert_eq!(96, buf.len());
    assert_eq!(&first_offset[0..32], &buf[32 * 0..32 * 1]);
    assert_eq!(&first_len[0..32],    &buf[32 * 1..32 * 2]);
    assert_eq!(&first_data[0..32],   &buf[32 * 2..32 * 3]);
}

#[test]
#[rustfmt::skip]
fn string_test() {
    let buf = Builder::new().add("random bytes".to_string()).build();
    let first_offset = hex::decode("0000000000000000000000000000000000000000000000000000000000000020").unwrap();
    let first_len = hex::decode("000000000000000000000000000000000000000000000000000000000000000c").unwrap();
    let first_data = hex::decode("72616e646f6d206279746573000000000000000000000000000000000000000000").unwrap();

    assert_eq!(96, buf.len());
    assert_eq!(&first_offset[0..32], &buf[32 * 0..32 * 1]);
    assert_eq!(&first_len[0..32],    &buf[32 * 1..32 * 2]);
    assert_eq!(&first_data[0..32],   &buf[32 * 2..32 * 3]);
}

#[test]
#[rustfmt::skip]
fn byte_n_test() {
    let buf = Builder::new().add(Bytes4([0xff; 4])).build();
    let first = hex::decode("ffffffff00000000000000000000000000000000000000000000000000000000").unwrap();

    assert_eq!(32, buf.len());
    assert_eq!(&first[0..32], &buf[0..32]);
}

#[test]
#[rustfmt::skip]
fn array_number_test() {
    let buf = Builder::new()
        .add([0xffu8 as i8; 2].to_vec())
        .add([0xffu8; 2].to_vec())
        .add([0xffffu16 as i16; 2].to_vec())
        .add([0xffffu16; 2].to_vec())
        .add([0xffffffffu32 as i32; 2].to_vec())
        .add([0xffffffffu32; 2].to_vec())
        .add([0xffffffffffffffffu64 as i64; 2].to_vec())
        .add([0xffffffffffffffffu64; 2].to_vec())
        .add([0xffffffffffffffffffffffffffffffffu128 as i128; 2].to_vec())
        .add([0xffffffffffffffffffffffffffffffffu128; 2].to_vec())
        .build();

    let i8_offset   = hex::decode("0000000000000000000000000000000000000000000000000000000000000140").unwrap();
    let u8_offset   = hex::decode("00000000000000000000000000000000000000000000000000000000000001a0").unwrap();
    let i16_offset  = hex::decode("0000000000000000000000000000000000000000000000000000000000000200").unwrap();
    let u16_offset  = hex::decode("0000000000000000000000000000000000000000000000000000000000000260").unwrap();
    let i32_offset  = hex::decode("00000000000000000000000000000000000000000000000000000000000002c0").unwrap();
    let u32_offset  = hex::decode("0000000000000000000000000000000000000000000000000000000000000320").unwrap();
    let i64_offset  = hex::decode("0000000000000000000000000000000000000000000000000000000000000380").unwrap();
    let u64_offset  = hex::decode("00000000000000000000000000000000000000000000000000000000000003e0").unwrap();
    let i128_offset = hex::decode("0000000000000000000000000000000000000000000000000000000000000440").unwrap();
    let u128_offset = hex::decode("00000000000000000000000000000000000000000000000000000000000004a0").unwrap();
    let i8_len      = hex::decode("0000000000000000000000000000000000000000000000000000000000000002").unwrap();
    let i8_1        = hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
    let i8_2        = hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
    let u8_len      = hex::decode("0000000000000000000000000000000000000000000000000000000000000002").unwrap();
    let u8_1        = hex::decode("00000000000000000000000000000000000000000000000000000000000000ff").unwrap();
    let u8_2        = hex::decode("00000000000000000000000000000000000000000000000000000000000000ff").unwrap();
    let i16_len     = hex::decode("0000000000000000000000000000000000000000000000000000000000000002").unwrap();
    let i16_1       = hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
    let i16_2       = hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
    let u16_len     = hex::decode("0000000000000000000000000000000000000000000000000000000000000002").unwrap();
    let u16_1       = hex::decode("000000000000000000000000000000000000000000000000000000000000ffff").unwrap();
    let u16_2       = hex::decode("000000000000000000000000000000000000000000000000000000000000ffff").unwrap();
    let i32_len     = hex::decode("0000000000000000000000000000000000000000000000000000000000000002").unwrap();
    let i32_1       = hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
    let i32_2       = hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
    let u32_len     = hex::decode("0000000000000000000000000000000000000000000000000000000000000002").unwrap();
    let u32_1       = hex::decode("00000000000000000000000000000000000000000000000000000000ffffffff").unwrap();
    let u32_2       = hex::decode("00000000000000000000000000000000000000000000000000000000ffffffff").unwrap();
    let i64_len     = hex::decode("0000000000000000000000000000000000000000000000000000000000000002").unwrap();
    let i64_1       = hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
    let i64_2       = hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
    let u64_len     = hex::decode("0000000000000000000000000000000000000000000000000000000000000002").unwrap();
    let u64_1       = hex::decode("000000000000000000000000000000000000000000000000ffffffffffffffff").unwrap();
    let u64_2       = hex::decode("000000000000000000000000000000000000000000000000ffffffffffffffff").unwrap();
    let i128_len    = hex::decode("0000000000000000000000000000000000000000000000000000000000000002").unwrap();
    let i128_1      = hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
    let i128_2      = hex::decode("ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();
    let u128_len    = hex::decode("0000000000000000000000000000000000000000000000000000000000000002").unwrap();
    let u128_1      = hex::decode("00000000000000000000000000000000ffffffffffffffffffffffffffffffff").unwrap();
    let u128_2      = hex::decode("00000000000000000000000000000000ffffffffffffffffffffffffffffffff").unwrap();

    // assert_eq!(192, buf.len());
    assert_eq!(&i8_offset[0..32],   &buf[32 * 0..32  * 1]);
    assert_eq!(&u8_offset[0..32],   &buf[32 * 1..32  * 2]);
    assert_eq!(&i16_offset[0..32],  &buf[32 * 2..32  * 3]);
    assert_eq!(&u16_offset[0..32],  &buf[32 * 3..32  * 4]);
    assert_eq!(&i32_offset[0..32],  &buf[32 * 4..32  * 5]);
    assert_eq!(&u32_offset[0..32],  &buf[32 * 5..32  * 6]);
    assert_eq!(&i64_offset[0..32],  &buf[32 * 6..32  * 7]);
    assert_eq!(&u64_offset[0..32],  &buf[32 * 7..32  * 8]);
    assert_eq!(&i128_offset[0..32], &buf[32 * 8..32  * 9]);
    assert_eq!(&u128_offset[0..32], &buf[32 * 9..32  * 10]);
    assert_eq!(&i8_len[0..32],      &buf[32 * 10..32 * 11]);
    assert_eq!(&i8_1[0..32],        &buf[32 * 11..32 * 12]);
    assert_eq!(&i8_2[0..32],        &buf[32 * 12..32 * 13]);
    assert_eq!(&u8_len[0..32],      &buf[32 * 13..32 * 14]);
    assert_eq!(&u8_1[0..32],        &buf[32 * 14..32 * 15]);
    assert_eq!(&u8_2[0..32],        &buf[32 * 15..32 * 16]);
    assert_eq!(&i16_len[0..32],     &buf[32 * 16..32 * 17]);
    assert_eq!(&i16_1[0..32],       &buf[32 * 17..32 * 18]);
    assert_eq!(&i16_2[0..32],       &buf[32 * 18..32 * 19]);
    assert_eq!(&u16_len[0..32],     &buf[32 * 19..32 * 20]);
    assert_eq!(&u16_1[0..32],       &buf[32 * 20..32 * 21]);
    assert_eq!(&u16_2[0..32],       &buf[32 * 21..32 * 22]);
    assert_eq!(&i32_len[0..32],     &buf[32 * 22..32 * 23]);
    assert_eq!(&i32_1[0..32],       &buf[32 * 23..32 * 24]);
    assert_eq!(&i32_2[0..32],       &buf[32 * 24..32 * 25]);
    assert_eq!(&u32_len[0..32],     &buf[32 * 25..32 * 26]);
    assert_eq!(&u32_1[0..32],       &buf[32 * 26..32 * 27]);
    assert_eq!(&u32_2[0..32],       &buf[32 * 27..32 * 28]);
    assert_eq!(&i64_len[0..32],     &buf[32 * 28..32 * 29]);
    assert_eq!(&i64_1[0..32],       &buf[32 * 29..32 * 30]);
    assert_eq!(&i64_2[0..32],       &buf[32 * 30..32 * 31]);
    assert_eq!(&u64_len[0..32],     &buf[32 * 31..32 * 32]);
    assert_eq!(&u64_1[0..32],       &buf[32 * 32..32 * 33]);
    assert_eq!(&u64_2[0..32],       &buf[32 * 33..32 * 34]);
    assert_eq!(&i128_len[0..32],    &buf[32 * 34..32 * 35]);
    assert_eq!(&i128_1[0..32],      &buf[32 * 35..32 * 36]);
    assert_eq!(&i128_2[0..32],      &buf[32 * 36..32 * 37]);
    assert_eq!(&u128_len[0..32],    &buf[32 * 37..32 * 38]);
    assert_eq!(&u128_1[0..32],      &buf[32 * 38..32 * 39]);
    assert_eq!(&u128_2[0..32],      &buf[32 * 39..32 * 40]);
}

#[test]
#[rustfmt::skip]
fn bytes_array_test() {
    let buf = Builder::new()
        .add(vec![Bytes(&[0xaau8; 16]), Bytes(&[0xffu8; 16])])
        .add(vec!["random string".to_string(), "what about another one".to_string()])
        .build();

    let bytes_array_offset    = hex::decode("0000000000000000000000000000000000000000000000000000000000000040").unwrap();
    let string_array_offset   = hex::decode("0000000000000000000000000000000000000000000000000000000000000120").unwrap();
    let bytes_array_len       = hex::decode("0000000000000000000000000000000000000000000000000000000000000002").unwrap();
    let bytes_array_1_offset  = hex::decode("0000000000000000000000000000000000000000000000000000000000000040").unwrap();
    let bytes_array_2_offset  = hex::decode("0000000000000000000000000000000000000000000000000000000000000080").unwrap();
    let bytes_array_1_len     = hex::decode("0000000000000000000000000000000000000000000000000000000000000010").unwrap();
    let bytes_array_1_bytes   = hex::decode("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00000000000000000000000000000000").unwrap();
    let bytes_array_2_len     = hex::decode("0000000000000000000000000000000000000000000000000000000000000010").unwrap();
    let bytes_array_2_bytes   = hex::decode("ffffffffffffffffffffffffffffffff00000000000000000000000000000000").unwrap();
    let string_array_len      = hex::decode("0000000000000000000000000000000000000000000000000000000000000002").unwrap();
    let string_array_1_offset = hex::decode("0000000000000000000000000000000000000000000000000000000000000040").unwrap();
    let string_array_2_offset = hex::decode("0000000000000000000000000000000000000000000000000000000000000080").unwrap();
    let string_array_1_len    = hex::decode("000000000000000000000000000000000000000000000000000000000000000d").unwrap();
    let string_array_1_bytes  = hex::decode("72616e646f6d20737472696e6700000000000000000000000000000000000000").unwrap();
    let string_array_2_len    = hex::decode("0000000000000000000000000000000000000000000000000000000000000016").unwrap();
    let string_array_2_bytes  = hex::decode("776861742061626f757420616e6f74686572206f6e6500000000000000000000").unwrap();

    assert_eq!(&bytes_array_offset[0..32],    &buf[32 * 0..32  * 1]);
    assert_eq!(&string_array_offset[0..32],   &buf[32 * 1..32  * 2]);
    assert_eq!(&bytes_array_len[0..32],       &buf[32 * 2..32  * 3]);
    assert_eq!(&bytes_array_1_offset[0..32],  &buf[32 * 3..32  * 4]);
    assert_eq!(&bytes_array_2_offset[0..32],  &buf[32 * 4..32  * 5]);
    assert_eq!(&bytes_array_1_len[0..32],     &buf[32 * 5..32  * 6]);
    assert_eq!(&bytes_array_1_bytes[0..32],   &buf[32 * 6..32  * 7]);
    assert_eq!(&bytes_array_2_len[0..32],     &buf[32 * 7..32  * 8]);
    assert_eq!(&bytes_array_2_bytes[0..32],   &buf[32 * 8..32  * 9]);
    assert_eq!(&string_array_len[0..32],      &buf[32 * 9..32  * 10]);
    assert_eq!(&string_array_1_offset[0..32], &buf[32 * 10..32 * 11]);
    assert_eq!(&string_array_2_offset[0..32], &buf[32 * 11..32 * 12]);
    assert_eq!(&string_array_1_len[0..32],    &buf[32 * 12..32 * 13]);
    assert_eq!(&string_array_1_bytes[0..32],  &buf[32 * 13..32 * 14]);
    assert_eq!(&string_array_2_len[0..32],    &buf[32 * 14..32 * 15]);
    assert_eq!(&string_array_2_bytes[0..32],  &buf[32 * 15..32 * 16]);
}

#[test]
#[rustfmt::skip]
fn address_test() -> Result<(), anyhow::Error> {
    let buf = Builder::new().add(Address::try_from(&[0xffu8; 20][..])?).build();

    let address = hex::decode("000000000000000000000000ffffffffffffffffffffffffffffffffffffffff").unwrap();

    assert_eq!(&address[0..32], &buf[32 * 0..32 * 1]);

    Ok(())
}

#[test]
#[rustfmt::skip]
fn function_test() -> Result<(), anyhow::Error> {
    let buf = Builder::new().add(Function::try_from(&[0xffu8; 24][..])?).build();

    let function = hex::decode("0000000000000000ffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();

    assert_eq!(&function[0..32], &buf[32 * 0..32 * 1]);

    Ok(())
}

#[test]
#[rustfmt::skip]
fn hex_test() -> Result<(), anyhow::Error> {
    let buf = Builder::new().add(Function::try_from("0xffffffffffffffffffffffffffffffffffffffffffffffff")?).build();

    let function = hex::decode("0000000000000000ffffffffffffffffffffffffffffffffffffffffffffffff").unwrap();

    assert_eq!(&function[0..32], &buf[32 * 0..32 * 1]);

    Ok(())
}

#[test]
#[rustfmt::skip]
fn tuple_test() -> Result<(), anyhow::Error> {
    let buf = Builder::new().add((0xffu8, 0xaabbu16)).build();

    let offset = hex::decode("0000000000000000000000000000000000000000000000000000000000000020").unwrap();
    let tuple1 = hex::decode("00000000000000000000000000000000000000000000000000000000000000ff").unwrap();
    let tuple2 = hex::decode("000000000000000000000000000000000000000000000000000000000000aabb").unwrap();

    assert_eq!(&offset[0..32], &buf[32 * 0..32 * 1]);
    assert_eq!(&tuple1[0..32], &buf[32 * 1..32 * 2]);
    assert_eq!(&tuple2[0..32], &buf[32 * 2..32 * 3]);

    Ok(())
}
