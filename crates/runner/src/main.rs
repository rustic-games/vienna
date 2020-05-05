#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use engine::Engine;

fn main() -> anyhow::Result<()> {
    println!("Hello, from runner!");

    let mut engine = Engine::builder().with_plugin_path("plugins").build()?;

    engine.run()?;

    Ok(println!("success"))
}
