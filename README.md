# Vienna

Experimental game engine for visual novels.

## Running locally

During early development, you can run the test setup as follows:

1. Install [Rust][install-rust].
2. Clone project.
3. Build test plugin.
4. Run test game.

on macOS, this will get you started:

```shell
curl --proto "=https" --tlsv1.2 -sSf "https://sh.rustup.rs" | sh
git clone "https://github.com/rustic-games/vienna"
cargo build --target "wasm32-wasi" --manifest-path "plugins/test/Cargo.toml"
cargo run
```

[install-rust]: https://rustup.rs/

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
