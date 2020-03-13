<!-- Version -->
<a href="https://crates.io/crates/solid">
<img src="https://img.shields.io/crates/v/solid.svg?style=flat-square"
alt="Crates.io version" />
</a>
<!-- Downloads -->
<a href="https://crates.io/crates/solid">
<img src="https://img.shields.io/crates/d/solid.svg?style=flat-square"
    alt="Download" />
</a>
<!-- Docs -->
<a href="https://docs.rs/solid">
<img src="https://img.shields.io/badge/docs-latest-blue.svg?style=flat-square"
    alt="docs.rs docs" />
</a>

### Solidity

Encoding/Decoding crate for the Solidity ABI. Used when making function calls
and/or decoding function call responses.

```rust
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
// macro, or use the `solid::Builder` manually.
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

// Use the `#[solid(constructor)]` attribute to declare a struct as a constructor.
// This is important because constructors do not have the function name prefix,
// unlike all other functions. Usually the struct name is used as the function
// name. To rename the function use the `#[solid(name = "<function_name>")]`
// where `<function_name>` is the name of your function.
// ie. `#[solid(name = "transfer")]`.
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
#[solid(error)]
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
// macro, or use the `solid::Builder` manually.
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
```

### [num_bigint](https://docs.rs/num-bigint/0.2.6/num_bigint/) Support

If you'd like support for `num_bigint` enable the `bigint` feature.

``` rust
// Note: BigInt is variable sized and encodes to `int256`.
// To encode to `uint256` use the `BigUint` struct.
// Also, BigInt supports numbers larger than the max value a uint256 can store, so the value
// will be truncated to 32 bytes before it's encoded.
#[derive(Encode)]
#[solid(rename = "transfer")]
struct ContractTransfer<'a> {
    amount: BigInt,
    to: &'a str
}
```

### Install

```toml
# Cargo.toml

# Default features which includes `derive`, and `serde`
solid = "0.1.0"

# num_bigint support
solid = { version = "0.1.0", default-features = false, features = [ "derive", "serde", "bigint" ] }
```

#### Features
 - derive: Add support for the `Encode` and `Decode` derive macros. (Recommended)
 - serde: Add support for `serde`s `Serialize` and `Deserialize` derive macros, and `to_bytes` function.
 - bigint: Add suport for `num_bigint` crate.

### cargo-solid Subcommand

cargo-solid is a cargo subcommand that allows you to generate a rust definition
of a solidity contract using the solidity abi output.

#### Subcommand Install

``` bash
cargo install cargo-solid
```

The following command generates the solidity abi for a contract.
``` bash
solc --combined-json abi solidity_contract.sol > solidity_contract.json
```

Then run the following command to generate the rust definition.
``` bash
cargo solid solidity_contract.json
```

This the output file will the be same name as the input file, and will be
located in `<the-current-directory>/src`. You can easily move the file if you'd
like, but either way you'll need to add `mod solidity_contract;` to your
`lib.rs` or `main.rs` files to use the generated code. If an example usage of
the `cargo-solid` command is located in the `examples/cargo-solid-example` directory.
