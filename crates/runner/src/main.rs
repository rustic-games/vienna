//! The binary used to run Vienna games.

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
    future_incompatible,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rustdoc,
    unused
)]
#![allow(
    clippy::float_arithmetic,
    clippy::multiple_crate_versions,
    clippy::implicit_return,
    clippy::shadow_reuse
)]

use engine::Engine;

fn main() -> anyhow::Result<()> {
    Engine::builder()
        .with_plugin_path("plugins")
        .with_window_dimensions(1440, 900)
        // Currently broken.
        //
        // see: https://git.io/Jfzlh
        //
        // .with_maximum_fps(90)
        .with_vsync()
        .build()?
        .run()
        .map_err(Into::into)
}
