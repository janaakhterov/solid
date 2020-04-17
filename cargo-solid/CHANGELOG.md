# CHANGELOG

## 0.1.4

### Added

  * Create generated files in a given directory other than `./src`; generate the `mod.rs` file.

  * Generate code for multiple contracts at once

  * Nightly option to use nightly feature when generating code; 
    use `BytesFix<N>` and `Int<N, M>` instead of the stable counterparts

### Fixed

  * Use spaces instead of tabs in generated code so formatting always looks right

## 0.1.3

### Added

  * Support for generating type definitions of return types [example](../examples/cargo-solid-example/src/stateful.rs)

  * Support for `bytesN`, `address`, and `function` Solidity types

### Misc

  * Convert function names to snake_case in generated code

  * Mark generated code as dead code to prevent warnings

  * Clean up code generation

## 0.1.2

### Misc

  * Removed `stateful.rs`
