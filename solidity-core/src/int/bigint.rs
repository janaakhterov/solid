use crate::{
    decode::Decode,
    encode::Encode,
    into_type::IntoType,
};
use num_bigint::{
    BigInt,
    BigUint,
};

impl Encode for BigInt {
    fn encode(self) -> Vec<u8> {
        let mut bytes = self.to_signed_bytes_be();

        let sign = (bytes[0] & 0x80) as u8;

        match bytes.len() {
            len if len < 32 => {
                let mut vec = vec![sign; 32];
                vec[32 - len..].copy_from_slice(&bytes);
                vec
            }

            32 => bytes,

            len => bytes.split_off(len - 32),
        }
    }

    fn required_len(&self) -> u64 {
        32
    }

    fn is_dynamic() -> bool {
        false
    }
}

impl<'a> Decode<'a> for BigInt {
    fn decode(buf: &[u8]) -> Self {
        BigInt::from_signed_bytes_be(&buf)
    }
}

impl IntoType for BigInt {
    fn into_type() -> String {
        "int256".to_string()
    }
}

impl Encode for BigUint {
    fn encode(self) -> Vec<u8> {
        let mut bytes = self.to_bytes_be();

        match bytes.len() {
            len if len < 32 => {
                let mut vec = vec![0u8; 32];
                vec[32 - len..].copy_from_slice(&bytes);
                vec
            }

            32 => bytes,

            len => bytes.split_off(len - 32),
        }
    }

    fn required_len(&self) -> u64 {
        32
    }

    fn is_dynamic() -> bool {
        false
    }
}

impl<'a> Decode<'a> for BigUint {
    fn decode(buf: &[u8]) -> Self {
        BigUint::from_bytes_be(&buf)
    }
}

impl IntoType for BigUint {
    fn into_type() -> String {
        "uint256".to_string()
    }
}
