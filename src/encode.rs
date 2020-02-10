pub trait Encode {
    fn encode(self) -> Vec<u8>;
    fn required_len(&self) -> u64;
    fn is_dynamic() -> bool;
}

impl<T> Encode for Vec<T>
where
    T: Encode,
{
    fn encode(self) -> Vec<u8> {
        let len = self.required_len();

        let mut buf = vec![0u8; len as usize];
        buf[24..32].copy_from_slice(&(self.len() as u64).to_be_bytes());

        let mut offset = self.len() * 32;

        for (index, bytes) in self.into_iter().map(Encode::encode).enumerate() {
            if T::is_dynamic() {
                buf[32 + index * 32 + 24..32 + (index + 1) * 32]
                    .copy_from_slice(&(offset as u64).to_be_bytes());
                buf[32 + offset..32 + offset + bytes.len()].copy_from_slice(&bytes);
                offset += bytes.len()
            } else {
                buf[32 + index * 32..32 + (index + 1) * 32].copy_from_slice(&bytes);
            }
        }

        buf
    }

    fn required_len(&self) -> u64 {
        self.iter().map(Encode::required_len).sum::<u64>()
            + if T::is_dynamic() {
                32 * self.len() + 32
            } else {
                32
            } as u64
    }

    fn is_dynamic() -> bool {
        true
    }
}
