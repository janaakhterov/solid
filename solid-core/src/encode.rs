/// Declares a type to be encodable as a Solidity type
pub trait Encode {
    fn encode(&self) -> Vec<u8>;

    /// The number of bytes required to encode the current type.
    ///
    /// This can be a static or dynamic value based on the type. The value should be 32 bytes
    /// aligned, and does not include the 32 bytes required for the offset.
    fn required_len(&self) -> u64 {
        32
    }

    /// Is this type considered `dynamic` by solidity
    ///
    /// If the type is dynamic then the `value` of this field is actually the offset
    fn is_dynamic() -> bool {
        false
    }
}

impl<T> Encode for &T
where
    T: Encode,
{
    fn encode(&self) -> Vec<u8> {
        T::encode(self)
    }

    fn required_len(&self) -> u64 {
        T::required_len(self)
    }

    fn is_dynamic() -> bool {
        T::is_dynamic()
    }
}

impl<T> Encode for Vec<T>
where
    T: Encode,
{
    fn encode(&self) -> Vec<u8> {
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

impl<'a, T> Encode for &'a [T]
where
    T: Encode,
{
    fn encode(&self) -> Vec<u8> {
        let len = self.required_len();

        let mut buf = vec![0u8; len as usize];
        buf[24..32].copy_from_slice(&(self.len() as u64).to_be_bytes());

        let mut offset = self.len() * 32;

        for (index, bytes) in (*self).into_iter().map(Encode::encode).enumerate() {
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
