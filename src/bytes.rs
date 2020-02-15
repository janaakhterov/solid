use crate::{
    decode::Decode,
    encode::Encode,
};

pub struct Bytes<'a>(pub &'a [u8]);

impl<'a> Encode for Bytes<'a> {
    fn encode(self) -> Vec<u8> {
        let len = self.required_len();
        let mut buf = vec![0u8; len as usize];
        buf[24..32].copy_from_slice(&(self.0.len() as u64).to_be_bytes());
        buf[32..32 + self.0.len()].copy_from_slice(&self.0);
        buf
    }

    fn required_len(&self) -> u64 {
        (if self.0.len() / 32 == 0 {
            32 + 32
        } else {
            (self.0.len() / 32 + 1) * 32 + 32
        }) as u64
    }

    fn is_dynamic() -> bool {
        true
    }
}

impl<'a> Decode<'a> for Bytes<'a> {
    fn decode(buf: &'a [u8]) -> Bytes<'a> {
        let len = u64::decode(&buf[0..32]);
        Bytes(&buf[32..32 + len as usize])
    }
}

#[cfg(feature = "derive")]
impl<'a> serde::ser::Serialize for Bytes<'a> {
    fn serialize<S: serde::ser::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_bytes(&self.0)
    }
}
