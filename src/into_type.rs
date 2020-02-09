use crate::{
    bytes::Bytes,
    bytesfix::*,
};

pub trait IntoType {
    fn into_type() -> String;
}

impl IntoType for i8 {
    fn into_type() -> String {
        "int8".to_string()
    }
}

impl IntoType for u8 {
    fn into_type() -> String {
        "uint8".to_string()
    }
}

impl IntoType for i16 {
    fn into_type() -> String {
        "int16".to_string()
    }
}

impl IntoType for u16 {
    fn into_type() -> String {
        "uint16".to_string()
    }
}

impl IntoType for i32 {
    fn into_type() -> String {
        "int32".to_string()
    }
}

impl IntoType for u32 {
    fn into_type() -> String {
        "uint32".to_string()
    }
}

impl IntoType for i64 {
    fn into_type() -> String {
        "int64".to_string()
    }
}

impl IntoType for u64 {
    fn into_type() -> String {
        "uint64".to_string()
    }
}

impl IntoType for i128 {
    fn into_type() -> String {
        "int128".to_string()
    }
}

impl IntoType for u128 {
    fn into_type() -> String {
        "uint128".to_string()
    }
}

impl IntoType for Bytes {
    fn into_type() -> String {
        "bytes".to_string()
    }
}

impl IntoType for String {
    fn into_type() -> String {
        "string".to_string()
    }
}

macro_rules! impl_into_type_bytesfix {
    ($ty: ty, $expr: expr) => {
        impl IntoType for $ty {
            fn into_type() -> String {
                ($expr).to_string()
            }
        }
    };
}

impl_into_type_bytesfix!(Bytes1, "bytes1");
impl_into_type_bytesfix!(Bytes2, "bytes2");
impl_into_type_bytesfix!(Bytes3, "bytes3");
impl_into_type_bytesfix!(Bytes4, "bytes4");
impl_into_type_bytesfix!(Bytes5, "bytes5");
impl_into_type_bytesfix!(Bytes6, "bytes6");
impl_into_type_bytesfix!(Bytes7, "bytes7");
impl_into_type_bytesfix!(Bytes8, "bytes8");
impl_into_type_bytesfix!(Bytes9, "bytes9");
impl_into_type_bytesfix!(Bytes10, "bytes10");
impl_into_type_bytesfix!(Bytes11, "bytes11");
impl_into_type_bytesfix!(Bytes12, "bytes12");
impl_into_type_bytesfix!(Bytes13, "bytes13");
impl_into_type_bytesfix!(Bytes14, "bytes14");
impl_into_type_bytesfix!(Bytes15, "bytes15");
impl_into_type_bytesfix!(Bytes16, "bytes16");
impl_into_type_bytesfix!(Bytes17, "bytes17");
impl_into_type_bytesfix!(Bytes18, "bytes18");
impl_into_type_bytesfix!(Bytes19, "bytes19");
impl_into_type_bytesfix!(Bytes20, "bytes20");
impl_into_type_bytesfix!(Bytes21, "bytes21");
impl_into_type_bytesfix!(Bytes22, "bytes22");
impl_into_type_bytesfix!(Bytes23, "bytes23");
impl_into_type_bytesfix!(Bytes24, "bytes24");
impl_into_type_bytesfix!(Bytes25, "bytes25");
impl_into_type_bytesfix!(Bytes26, "bytes26");
impl_into_type_bytesfix!(Bytes27, "bytes27");
impl_into_type_bytesfix!(Bytes28, "bytes28");
impl_into_type_bytesfix!(Bytes29, "bytes29");
impl_into_type_bytesfix!(Bytes30, "bytes30");
impl_into_type_bytesfix!(Bytes31, "bytes31");
impl_into_type_bytesfix!(Bytes32, "bytes32");

impl<T> IntoType for Vec<T>
where
    T: IntoType,
{
    fn into_type() -> String {
        format!("{}[]", T::into_type())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn type_test() {
        assert_eq!("uint8[]", Vec::<u8>::into_type());
        assert_eq!("bytes[]", Vec::<Bytes>::into_type());
        assert_eq!("bytes[][]", Vec::<Vec::<Bytes>>::into_type());
    }
}
