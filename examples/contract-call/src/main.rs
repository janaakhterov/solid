#![allow(dead_code)]
use num_bigint::{
    BigInt,
    ToBigInt,
};
use serde::{
    Deserialize,
    Serialize,
};
use solid::{
    derive::{
        Decode,
        Encode,
    },
    to_bytes,
    Address,
    Builder,
    Bytes,
    Bytes10,
    Decode,
    Encode,
    Result,
    Uint256,
};

// Basic usage using the built in `Encode` derive macro.
// (Requires the `derive` feature.)
#[derive(Encode)]
struct ContractCallEncode<'a> {
    pub name: &'a str,
    pub number: u128,
    pub bytes10: Bytes10,
    pub bytes: Bytes<'a>,
}

// Basic usage using serde. (Requires the `serde` feature).
// Note: Serde only supports a subset of the types that Solidity supports.
// If you need to support more types you'll have to use the `Encode` derive
// macro, or use the `solidity::Builder` manually.
#[derive(Serialize)]
pub struct ContractCallSerde<'a> {
    // String is also supported, but it's recommened you use &str when possible.
    // pub name: String,
    pub name: &'a str,
    pub number: u128,
    pub bytes: Bytes<'a>,
    // Bytes10 cannot be serialized correctly using serde.
    // pub bytes: Bytes10,
}

// Use the `#[solidity(constructor)]` attribute to declare a struct as a constructor.
// This is important because constructors do not have the function name prefix,
// unlike all other functions. Usually the struct name is used as the function
// name. To rename the function use the `#[solidity(name = "<function_name>")]`
// where `<function_name>` is the name of your function.
// ie. `#[solidity(name = "transfer")]`.
#[derive(Encode)]
struct ContractConstructorEncode<'a> {
    pub value: u128,
    pub string: &'a str,
}

// Basic usage with the built in `Decode` derive macro.
// (Requires the `derive` feature.)
// Note: `Uint256` and all other `Int`/`Uint` types are simple
// wrappers around `[u8; 32]`. The point of them is to support all
// `int`/`uint` Solidity types.
#[derive(Decode)]
struct ContractCallResponse<'a> {
    int: Uint256,
    // Note: &'a [u8] is *not* the same as `Bytes<'a>`. The former is is `uint8[]` in solidity
    // while the latter is `bytes`. The two types are encoded very differently so decoding
    // `bytes` as `uint8[]` array will give you invalid data if not fail outright.
    bytes: Bytes<'a>,
    memo: &'a str,
    address: Address,
}

// Basic usage with serde's `Deserialize` derive macro.
// (Requires the `serde` feature.)
// Note: Serde only supports a subset of the types that Solidity supports.
// If you need to support more types you'll have to use the `Encode` derive
// macro, or use the `solidity::Builder` manually.
#[derive(Deserialize)]
struct ContractCallResponseSerde<'a> {
    int: u128,
    bytes: &'a [u8],
    memo: &'a str,
    // There is no way to read `Address` with serde.
    // address: Address
}

// Support for composite types and `Vec`
#[derive(Encode)]
struct ContractCallComposite<'a> {
    to: (&'a str, u128),
    memos: &'a [&'a str],
    matrix: &'a [&'a [&'a [u8]]],
}

// Note: BigInt is variable sized and encodes to `int256`.
// To encode to `uint256` use the `BigUint` struct.
// Also, BigInt supports numbers larger than the max value a uint256 can store, so the value
// will be truncated to 32 bytes before it's encoded.
#[derive(Encode)]
#[solidity(rename = "transfer")]
struct ContractTransfer<'a> {
    amount: BigInt,
    to: &'a str,
}

pub fn main() -> Result<()> {
    // Uses `derive::Encode`
    let call_encode = ContractCallEncode {
        name: "daniel",
        number: 10,
        bytes10: Bytes10([1u8; 10]),
        bytes: Bytes(&[0xffu8; 53]),
    };

    let _bytes = call_encode.encode();

    // Uses `serde::Serialize`
    let call_serialize = ContractCallSerde {
        name: "daniel",
        number: 10,
        // Unsupported by serde
        // bytes10: Bytes10([1u8; 10]),
        bytes: Bytes(&[0xffu8; 53]),
    };

    // Call `to_bytes(<struct that implements serde::Deserialize>)`
    to_bytes(&call_serialize)?;

    // Uses `serde_derive::Serialize`
    let constructor = ContractConstructorEncode {
        value: 10,
        string: "just a random string",
    };

    let _constructor_bytes = constructor.encode();

    // Manually construct the function
    let _function = Builder::new()
        .name("transfer")
        .push("daniel")
        .push(10u128)
        .push(Bytes10([1u8; 10]))
        .build();

    // Example of the composite struct
    let composite = ContractCallComposite {
        to: ("daniel", 10u128),
        memos: &["This is the first memo.", "This is the second memo."],
        matrix: &[&[&[1, 2, 3], &[4, 5, 6]], &[&[7, 8, 9], &[10, 11, 12]]],
    };

    let _composite_bytes = composite.encode();

    let bigint = ContractTransfer {
        amount: 1_000_000_000.to_bigint().unwrap(),
        to: "daniel",
    };

    let _bigint_bytes = bigint.encode();

    Ok(())
}
