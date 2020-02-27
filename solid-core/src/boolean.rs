use crate::{
    decode::Decode,
    encode::Encode,
};

impl Encode for bool {
    fn encode(&self) -> Vec<u8> {
        let mut buf = vec![0u8; 32];
        buf[31] = if *self { 1 } else { 0 };
        buf
    }

    fn required_len(&self) -> u64 {
        32
    }

    fn is_dynamic() -> bool {
        false
    }
}

impl<'a> Decode<'a> for bool {
    fn decode(buf: &'a [u8]) -> Self {
        buf[31] == 1
    }
}
