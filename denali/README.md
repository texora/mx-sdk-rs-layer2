# denali

Rust implementation of the Denali smart contract test file format.

It is composed of 2 parts:
- the denali serde representation
- the standard denali value interpreter

Both of them are detailed under this specification: https://docs.dharitri.com/developers/developer-reference/denali-tests

This crate only deals with the format, not with its semantics or execution. For the execution engine, see `dharitri-wasm-debug/denali`. This also means that this crate does not and should not depend on any `dharitri-*` crate, it is the base format and nothing else.
