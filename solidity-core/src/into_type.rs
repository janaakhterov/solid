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

impl IntoType for String {
    fn into_type() -> String {
        "string".to_string()
    }
}

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
