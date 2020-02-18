use crate::encode::Encode;

pub trait Decode<'a>: Sized {
    fn decode(buf: &'a [u8]) -> Self;
}

impl<'a, T: Decode<'a> + Encode> Decode<'a> for Vec<T> {
    fn decode(buf: &'a [u8]) -> Self {
        let len = u64::decode(&buf[0..32]) as usize;

        let mut vec: Vec<T> = Vec::with_capacity(len);
        for index in 0..len {
            if T::is_dynamic() {
                vec.push(T::decode(&buf[32 + index * 32..32 + (index + 1) * 32]))
            } else {
                let offset = u64::decode(&buf[32 + index * 32..32 + (index + 1) * 32]) as usize;
                vec.push(T::decode(&buf[offset..]))
            }
        }
        vec
    }
}
