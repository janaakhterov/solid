use crate::{
    decode::Decode,
    encode::Encode,
    into_type::IntoType,
};
use ethereum_types::{
    Address,
    U128,
    U256,
};
use std::borrow::Cow;

impl Encode for U128 {
    fn encode(&self) -> Vec<u8> {
        self.as_u128().encode()
    }

    fn required_len(&self) -> u64 {
        self.as_u128().required_len()
    }

    fn is_dynamic() -> bool {
        u128::is_dynamic()
    }
}

impl<'a> Decode<'a> for U128 {
    fn decode(buf: &[u8]) -> Self {
        U128::from(u128::decode(buf))
    }
}

impl IntoType for U128 {
    fn into_type() -> Cow<'static, str> {
        u128::into_type()
    }
}

impl Encode for U256 {
    fn encode(&self) -> Vec<u8> {
        let mut buf = vec![0u8; 32];
        self.to_big_endian(&mut buf);
        buf
    }

    fn required_len(&self) -> u64 {
        32
    }

    fn is_dynamic() -> bool {
        false
    }
}

impl<'a> Decode<'a> for U256 {
    fn decode(buf: &[u8]) -> Self {
        U256::from(buf)
    }
}

impl IntoType for U256 {
    fn into_type() -> Cow<'static, str> {
        Cow::Borrowed("u256")
    }
}

impl Encode for Address {
    fn encode(&self) -> Vec<u8> {
        let mut buf = vec![0u8; 32];
        buf[12..].copy_from_slice(&self.as_bytes());
        buf
    }

    fn required_len(&self) -> u64 {
        32
    }

    fn is_dynamic() -> bool {
        false
    }
}

impl<'a> Decode<'a> for Address {
    fn decode(buf: &[u8]) -> Self {
        let mut address = [0u8; 20];
        address.copy_from_slice(&buf[12..32]);
        Address::from(address)
    }
}

impl IntoType for Address {
    fn into_type() -> Cow<'static, str> {
        Cow::Borrowed("address")
    }
}
