use crate::function::Builder;

#[test]
fn number_test() {
    let buf = Builder::new()
        .add_i8(0xffu8 as i8)
        .add_u8(0xffu8)
        .add_i16(0xffffu16 as i16)
        .add_u16(0xffffu16)
        .add_i32(0xffffffffu32 as i32)
        .add_u32(0xffffffffu32)
        .add_i64(0xffffffffffffffffu64 as i64)
        .add_u64(0xffffffffffffffffu64)
        .add_i128(0xffffffffffffffffffffffffffffffffu128 as i128)
        .add_u128(0xffffffffffffffffffffffffffffffffu128)
        .build();
    let first =
        hex::decode("00000000000000000000000000000000000000000000000000000000000000ff").unwrap();
    let second =
        hex::decode("00000000000000000000000000000000000000000000000000000000000000ff").unwrap();
    let third =
        hex::decode("000000000000000000000000000000000000000000000000000000000000ffff").unwrap();
    let forth =
        hex::decode("000000000000000000000000000000000000000000000000000000000000ffff").unwrap();
    let fifth =
        hex::decode("00000000000000000000000000000000000000000000000000000000ffffffff").unwrap();
    let sixth =
        hex::decode("00000000000000000000000000000000000000000000000000000000ffffffff").unwrap();
    let seventh =
        hex::decode("000000000000000000000000000000000000000000000000ffffffffffffffff").unwrap();
    let eighth =
        hex::decode("000000000000000000000000000000000000000000000000ffffffffffffffff").unwrap();
    let nineth =
        hex::decode("00000000000000000000000000000000ffffffffffffffffffffffffffffffff").unwrap();
    let tenth =
        hex::decode("00000000000000000000000000000000ffffffffffffffffffffffffffffffff").unwrap();

    assert_eq!(&first[0..32], &buf[0..32]);
    assert_eq!(&second[0..32], &buf[32..32 * 2]);
    assert_eq!(&third[0..32], &buf[32 * 2..32 * 3]);
    assert_eq!(&forth[0..32], &buf[32 * 3..32 * 4]);
    assert_eq!(&fifth[0..32], &buf[32 * 4..32 * 5]);
    assert_eq!(&sixth[0..32], &buf[32 * 5..32 * 6]);
    assert_eq!(&seventh[0..32], &buf[32 * 6..32 * 7]);
    assert_eq!(&eighth[0..32], &buf[32 * 7..32 * 8]);
    assert_eq!(&nineth[0..32], &buf[32 * 8..32 * 9]);
    assert_eq!(&tenth[0..32], &buf[32 * 9..32 * 10]);
}

#[test]
fn byte_test() {
    let buf = Builder::new().add_bytes("random bytes".as_bytes()).build();
    let first_offset =
        hex::decode("0000000000000000000000000000000000000000000000000000000000000020").unwrap();
    let first_len =
        hex::decode("000000000000000000000000000000000000000000000000000000000000000c").unwrap();
    let first_data =
        hex::decode("72616E646F6D206279746573000000000000000000000000000000000000000000").unwrap();

    assert_eq!(96, buf.len());
    assert_eq!(&first_offset[0..32], &buf[0..32]);
    assert_eq!(&first_len[0..32], &buf[32..32 * 2]);
    assert_eq!(&first_data[0..32], &buf[32 * 2..32 * 3]);
}

#[test]
fn string_test() {
    let buf = Builder::new().add_string("random bytes").build();
    let first_offset =
        hex::decode("0000000000000000000000000000000000000000000000000000000000000020").unwrap();
    let first_len =
        hex::decode("000000000000000000000000000000000000000000000000000000000000000c").unwrap();
    let first_data =
        hex::decode("72616E646F6D206279746573000000000000000000000000000000000000000000").unwrap();

    assert_eq!(96, buf.len());
    assert_eq!(&first_offset[0..32], &buf[0..32]);
    assert_eq!(&first_len[0..32], &buf[32..32 * 2]);
    assert_eq!(&first_data[0..32], &buf[32 * 2..32 * 3]);
}

#[test]
fn byte_n_test() {
    let buf = Builder::new().add_bytes_n(&[0xff; 4]).build();
    let first =
        hex::decode("ffffffff00000000000000000000000000000000000000000000000000000000").unwrap();

    assert_eq!(32, buf.len());
    assert_eq!(&first[0..32], &buf[0..32]);
}
