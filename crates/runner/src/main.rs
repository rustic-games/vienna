#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

use engine::Engine;

fn main() -> anyhow::Result<()> {
    println!("Hello, from runner!");

    let mut engine = Engine::builder()
        .with_plugin_path("plugins")
        .continuous()
        .build()?;

    engine.run()?;

    println!("success");
    Ok(())
}
