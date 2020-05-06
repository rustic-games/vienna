# Vienna

Experimental game engine for visual novels.

## Goal

This is a _work in progress_ game engine that focuses on making it simple but
flexible to write visual novels.

The goal is to allow writing a visual novel without having to write any code,
while also allowing more complex interactive novels by building game logic in
your programming language of choice.

## Concept

- Engine written in [Rust][]
- First class support for [WebAssembly][wasm] based plugins
- Games built using a storybook and plugins

[rust]: https://www.rust-lang.org/
[wasm]: https://webassembly.org/

## Development

During development, you can run the test setup as follows:

1. Install [Rust][install-rust].
2. Clone project.
3. Build test plugin.
4. Run test game.

on macOS, this will get you started:

```shell
curl --proto "=https" --tlsv1.2 -sSf "https://sh.rustup.rs" | sh
git clone "https://github.com/rustic-games/vienna"
cargo build --target "wasm32-unknown-unknown" --manifest-path "plugins/test/Cargo.toml"
cargo run
```

[install-rust]: https://rustup.rs/

## Crates

The project consists of separate crates, each with their own set of
responsibilities.

### Engine

The responsibility of the `vienna-engine` crate is to advance the game state
based on (player) input and render the results to the screen.

### Runner

The `vienna-runner` crate will become the binary to run the engine with a set of
external configuration files and assets. This allows you to distribute a visual
novel without having to compile the game itself.

Until then, the purpose of this crate is to allow running `cargo run` from the
project root to test the engine.

### SDK

The `vienna` crate provides a Software Development Kit interface to build
Wasm plugins in Rust.

While technically not required, the crate vastly simplifies the process of
creating a new plugin, and handles all unsafe _FFI_ operations needed to
communicate with the engine at runtime.

The crate has the naked `vienna` name, since it is the one most often used by
external developers.

## Plugins

A set of default engine plugins live in the [`plugins` directory](./plugins).

The engine embeds these plugins by default to provide the core functionality for
any game. Custom plugins can further extend the capability of a game.

Currently, a single `test` plugin exists to validate the functionality of the
engine.
