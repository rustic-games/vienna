#![warn(clippy::all)]
#![warn(clippy::pedantic)]

use engine::Engine;

fn main() -> anyhow::Result<()> {
    println!("Hello, from runner!");

    Engine::default().run()?;

    Ok(println!("success"))
}
