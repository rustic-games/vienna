#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod engine;
mod error;
mod plugin;
mod plugin_manager;

pub use engine::DefaultEngine as Engine;
pub use error::Error;
