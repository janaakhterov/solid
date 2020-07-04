# CHANGELOG

## 0.1.5

### Fix

  * `Function` was being encoded as `int192` instead of `bytes24`

## 0.1.4

### Add

  * Implement support several types from the `ethereum_types` crate

  * Support for using structs that implement `Encode` within other structs
    that implement `Encode`. Previously this was not possible and would require
    fields to be tuples.

### Fix

  * Bug requiring the import for traits `Encode` and `Decode` when using
    the derive macros

### Misc

  * Update IntoType to trait to use `Cow<'static, str>` instead of `String` 
    to remove potential allocations

## 0.1.3

### Misc

  * Rexported `BytesFix`

  * Rexported `Int` and `Uint`

## 0.1.2

### Added

  * `from_bytes()` for decoding Solidity response buffer

  * `nigthly` feature for experimental support of const generics with `BytesFix`, `Int`, and `Uint`

  * More documentation through the crate

### Removed

  * `Bytes<N>` from top level documentation.

  * `Int<N>` from top level documentation.

### Fixed

  * Spelling mistake in documentation
