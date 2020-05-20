#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    future_incompatible,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rustdoc,
    unused
)]
#![allow(clippy::multiple_crate_versions)]

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
