use crate::encode::Encode;

impl Encode for String {
    fn encode(self) -> Vec<u8> {
        let len = self.required_len();
        let mut buf = vec![0u8; len as usize];
        buf[24..32].copy_from_slice(&(self.len() as u64).to_be_bytes());
        buf[32..32+self.len()].copy_from_slice(self.as_bytes());
        buf
    }

    fn required_len(&self) -> u64 {
        (if self.len() / 32 == 0  {
            32 + 32
        } else {
            (self.len() / 32 + 1) * 32 + 32
        }) as u64
    }

    fn is_dynamic() -> bool {
        true
    }
}
