use crate::{
    decode::Decode,
    encode::Encode,
    into_type::IntoType,
};

impl Encode for String {
    fn encode(&self) -> Vec<u8> {
        let len = self.required_len();
        let mut buf = vec![0u8; len as usize];
        buf[24..32].copy_from_slice(&(self.len() as u64).to_be_bytes());
        buf[32..32 + self.len()].copy_from_slice(self.as_bytes());
        buf
    }

    fn required_len(&self) -> u64 {
        (if self.len() / 32 == 0 {
            32 + 32
        } else {
            (self.len() / 32 + 1) * 32 + 32
        }) as u64
    }

    fn is_dynamic() -> bool {
        true
    }
}

impl<'a> Decode<'a> for String {
    fn decode(buf: &'a [u8]) -> Self {
        let len = u64::decode(&buf[0..32]);
        String::from_utf8(buf[32..32 + len as usize].to_vec()).unwrap()
    }
}

impl IntoType for String {
    fn into_type() -> String {
        "string".to_string()
    }
}

impl Encode for &str {
    fn encode(&self) -> Vec<u8> {
        let len = self.required_len();
        let mut buf = vec![0u8; len as usize];
        buf[24..32].copy_from_slice(&(self.len() as u64).to_be_bytes());
        buf[32..32 + self.len()].copy_from_slice(self.as_bytes());
        buf
    }

    fn required_len(&self) -> u64 {
        (if self.len() / 32 == 0 {
            32 + 32
        } else {
            (self.len() / 32 + 1) * 32 + 32
        }) as u64
    }

    fn is_dynamic() -> bool {
        true
    }
}

impl IntoType for &str {
    fn into_type() -> String {
        "string".to_string()
    }
}

impl<'a> Decode<'a> for &'a str {
    fn decode(buf: &'a [u8]) -> Self {
        let len = u64::decode(&buf[0..32]);
        std::str::from_utf8(&buf[32..32 + len as usize]).unwrap()
    }
}
