### Vienna: SDK

The `vienna` crate provides a Software Development Kit interface to build
Wasm plugins in Rust.

While technically not required, the crate vastly simplifies the process of
creating a new plugin, and handles all unsafe _FFI_ operations needed to
communicate with the engine at runtime.

The crate has the naked `vienna` name, since it is the one most often used by
external developers.
