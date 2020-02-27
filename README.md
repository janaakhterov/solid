### Solidity

Encoding/Decoding crate for the Solidity ABI. Used when making function calls
and/or decoding function call responses.

```rust
use serde::{
    Deserialize,
    Serialize,
};
use solidity::{
    derive::{
        Decode,
        Encode,
    },
    Address,
    Builder,
    Bytes,
    Bytes10,
    Decode,
    Encode,
    Uint256,
};

// Basic usage using the built in `Encode` derive macro.
// (Requires the `derive` feature.)
#[derive(Encode)]
struct ContractCallEncode<'a> {
    pub name: String,
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
    pub name: String,
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
#[solidity(constructor)]
struct ContractConstructorEncode {
    pub value: u128,
    pub string: String,
}

// Basic usage with the built in `Decode` derive macro.
// (Requires the `derive` feature.)
// Note: `Uint256` and all other `Int`/`Uint` types are simple
// wrappers around `[u8; 32]`. The point of them is to support all
// `int`/`uint` Solidity types.
#[derive(Decode)]
struct ContractCallResponse<'a> {
    int: Uint256,
    bytes: Bytes<'a>,
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
    // There is no way to read `Address` with serde.
    // address: Address,
}

// Support for composite types and `Vec`
#[derive(Encode)]
struct ContractCallComposite {
    to: (String, u128),
    memos: Vec<String>,
    matrix: Vec<Vec<Vec<u8>>>,
}

```

### `num_bigint` Support

If you'd like support for `num_bigint` enable the `bigint` feature.

### `fixed` Support

If you'd like support for `fixed` enable the `fixed` feature.
