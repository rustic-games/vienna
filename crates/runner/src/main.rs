#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

use engine::Engine;

fn main() -> anyhow::Result<()> {
    Engine::builder()
        .with_plugin_path("plugins")
        .build()?
        .run()
        .map_err(Into::into)
}
