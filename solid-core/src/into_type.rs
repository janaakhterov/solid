use std::borrow::Cow;

pub trait IntoType {
    fn into_type() -> Cow<'static, str>;
}

impl IntoType for i8 {
    fn into_type() -> Cow<'static, str> {
        Cow::Borrowed("int8")
    }
}

impl IntoType for u8 {
    fn into_type() -> Cow<'static, str> {
        Cow::Borrowed("uint8")
    }
}

impl IntoType for i16 {
    fn into_type() -> Cow<'static, str> {
        Cow::Borrowed("int16")
    }
}

impl IntoType for u16 {
    fn into_type() -> Cow<'static, str> {
        Cow::Borrowed("uint16")
    }
}

impl IntoType for i32 {
    fn into_type() -> Cow<'static, str> {
        Cow::Borrowed("int32")
    }
}

impl IntoType for u32 {
    fn into_type() -> Cow<'static, str> {
        Cow::Borrowed("uint32")
    }
}

impl IntoType for i64 {
    fn into_type() -> Cow<'static, str> {
        Cow::Borrowed("int64")
    }
}

impl IntoType for u64 {
    fn into_type() -> Cow<'static, str> {
        Cow::Borrowed("uint64")
    }
}

impl IntoType for i128 {
    fn into_type() -> Cow<'static, str> {
        Cow::Borrowed("int128")
    }
}

impl IntoType for u128 {
    fn into_type() -> Cow<'static, str> {
        Cow::Borrowed("uint128")
    }
}

impl<'a, T> IntoType for &'a [T]
where
    T: IntoType,
{
    fn into_type() -> Cow<'static, str> {
        Cow::Owned(format!("{}[]", T::into_type()))
    }
}

impl<T> IntoType for Vec<T>
where
    T: IntoType,
{
    fn into_type() -> Cow<'static, str> {
        Cow::Owned(format!("{}[]", T::into_type()))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bytes::Bytes;

    #[test]
    fn type_test() {
        assert_eq!("uint8[]", Vec::<u8>::into_type());
        assert_eq!("bytes[]", Vec::<Bytes>::into_type());
        assert_eq!("bytes[][]", Vec::<Vec::<Bytes>>::into_type());
    }
}
