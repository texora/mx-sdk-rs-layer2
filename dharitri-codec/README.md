# dharitri-codec

Lightweight binary serializer/deserializer, written especially for Dharitri smart contracts.

Designed to:
- produce minimal WASM bytecode
- be fast
- avoid data copy as much as possible

Largely inspired by the Parity SCALE codec, but a completely different format and implementation.

For more info about the serialization format, see [the developer reference](https://docs.dharitri.com/developers/developer-reference/dharitri-serialization-format/).

# no-std

Being designed for dharitri-wasm smart contracts, it needs to be able to run in a no-std environment.
