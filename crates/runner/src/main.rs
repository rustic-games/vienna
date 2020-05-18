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
        .build()?
        .run()
        .map_err(Into::into)
}
