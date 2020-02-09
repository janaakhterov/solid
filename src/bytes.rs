use crate::encode::Encode;

pub struct Bytes(pub Vec<u8>);

impl Encode for Bytes {
    fn encode(self) -> Vec<u8> {
        let len = self.required_len();
        println!("[Encode] [Bytes] buf.len: {}", len);
        println!("[Encode] [Bytes] self.len: {}", self.0.len());
        let mut buf = vec![0u8; len as usize];
        buf[24..32].copy_from_slice(&(self.0.len() as u64).to_be_bytes());
        buf[32..32 + self.0.len()].copy_from_slice(&self.0);
        buf
    }

    fn required_len(&self) -> u64 {
        (if self.0.len() / 32 == 0  {
            32 + 32
        } else {
            (self.0.len() / 32 + 1) * 32 + 32
        }) as u64
    }

    fn is_dynamic() -> bool {
        true
    }
}
