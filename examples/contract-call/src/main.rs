use solidity::{
    Address,
    Bytes10,
    Encode,
    Decode,
    Uint256,
    to_bytes,
    derive::Encode,
    derive::Decode,
    Result,
};
use serde::{
    Deserialize,
    Serialize,
};

// Contract contstructors do not require a function identifier prefix
// so it's completely fine using serde for serialization here. Of course
// this does limit you to using only serde supported types.
#[derive(Serialize)]
pub struct ContractConstructor {
    pub value: u128,
    pub string: String,
}

#[derive(Encode)]
struct ContractCall {
    pub name: String,
    pub value: u128,
}

// You can either use serde for deserializing the response if the response
// consists of serde supported types, or you can use `solidity::Decode` for
// types that serde doesnt' support.
#[derive(Deserialize)]
pub struct SerdeContractResponse {
    pub value: u128,
    pub string: String,
}

// Use derive macro `Decode` if you need to support more types such as
// `Address`, `Bytes10`, or `u256`.
#[derive(Decode)]
struct DecodeContractResponse {
    pub value: Uint256,
    pub string: String,
    pub bytes10: Bytes10,
    pub address: Address,
}

pub fn main() -> Result<()> {
    // Uses `derive::Encode`
    let call = ContractCall {
        name: "transfer".to_string(),
        value: 10,
    };

    // Uses `serde_derive::Serialize`
    let constructor = ContractConstructor {
        value: 10,
        string: "just a random string".to_string(),
    };

    // Call `Encode::encode()` to get bytes
    let _bytes = call.encode();

    // Call `to_bytes(<struct that implements serde::Deserialize>)`
    to_bytes(&constructor)?;

    Ok(())
}
