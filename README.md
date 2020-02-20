### Solidity

Encoding/Decoding crate for the Solidity ABI. Used when making function calls
and/or decoding function call responses.

```rust
use solidity::{
    Address,
    Bytes10,
    derive::Decode,
    derive::Encode
};
use serde_derive::{
    Deserialize,
    Serialize,
};

// Contract contstructors do not require a function identifier prefix
// so it's completely fine using serde for serialization here. Of course
// this does limit you to using only serde supported types.
#[derive(Serialize)]
struct ContractConstructor {
    value: u128,
    string: String,
}

#[derive(Encode)]
struct ContractCall {
    #[function_name]
    name: String,
    value: u128,
    string: String,
}

// You can either use serde for deserializing the response if the response
// consists of serde supported types, or you can use `solidity::Decode` for
// types that serde doesnt' support.
#[derive(Deserailize)]
struct SerdeContractResponse {
    value: u128,
    string: String,
}

// Use derive macro `Decode` if you need to support more types such as
// `Address`, `Bytes10`, or `u256`.
#[derive(Decode)]
struct DecodeContractResponse {
    value: u256,
    string: String,
    bytes10: Bytes10,
    address: Address,
}

// Using the `unstable` feature
// Use derive macro `Decode` if you need to support more types such as
// `Address`, `BytesFix`, or `u256`.
#[derive(Decode)]
struct DecodeContractResponse {
    value: u256,
    string: String,
    bytes10: BytesFix<10>,
    address: Address,
}
```

### `num_bigint` Support

If you'd like support for `num_bigint` enable the `bigint` feature.

### `fixed` Support

If you'd like support for `fixed` enable the `fixed` feature.
