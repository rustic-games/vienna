# Vienna

Experimental game engine for visual novels.

## Goal

This is a _work in progress_ game engine that focuses on making it simple but
flexible to write visual novels.

The goal is to allow writing a visual novel without having to write any code,
while also allowing more complex interactive novels by building game logic in
your programming language of choice.

## Visual Novel

The term "visual novel" is somewhat broad and vague.

In Vienna's case, it represents the following characteristics:

1. The game provides a visual story.
2. The gameplay ranges from minimal to full interactivity.
3. You play a (non-linear) sequence of scenes.
4. The world operates in 2D space.
5. It does not require high-performance computing.

There are more genres that fit this description, and all are a focus point of
the engine:

- [Visual Novel](https://www.giantbomb.com/_/3015-2029)
- [Interactive Film](https://en.wikipedia.org/wiki/Interactive_film)
- [(Point & Click) Adventure Game](https://en.wikipedia.org/wiki/Adventure_game#Point-and-click_adventure_games)

The biggest inspiration for this engine is an all-time classic game: [Indiana
Jones and the Fate of Atlantis][indy].

There are others, such as [Myst], [Broken Age], [The Walking Dead], [The Longest
Journey], [Life is Strange], [Broken Sword], [Dreamfall], [The Curse of Monkey
Island], [Blade Runner], [Syberia] and [Starship Titanic].

Not all of these can be represented by this engine, but the goal is to cover as
much as possible, by offering a flexible system of _plugins_ and _widgets_.

[indy]: https://www.gog.com/game/indiana_jones_and_the_fate_of_atlantis
[myst]: https://www.gog.com/game/myst_masterpiece_edition
[broken age]: https://www.gog.com/game/broken_age
[the walking dead]: https://www.gog.com/game/walking_dead_season_1_the
[the longest journey]: https://www.gog.com/game/the_longest_journey
[life is strange]: https://en.wikipedia.org/wiki/Life_Is_Strange
[broken sword]: https://www.gog.com/game/broken_sword_directors_cut
[dreamfall]: https://www.gog.com/game/dreamfall_the_longest_journey
[the curse of monkey island]: https://www.gog.com/game/the_curse_of_monkey_island
[blade runner]: https://www.gog.com/game/blade_runner
[syberia]: https://www.gog.com/game/syberia
[starship titanic]: https://www.gog.com/game/starship_titanic

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
