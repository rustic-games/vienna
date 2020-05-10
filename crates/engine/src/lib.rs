#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

mod builder;
mod config;
mod engine;
mod error;
mod game_state;
mod plugin;
mod renderer;
mod updater;

use builder::Builder;
use game_state::GameState;
use renderer::Renderer;
use updater::Updater;

pub use error::Error;

/// A convenient top-level engine type exposed to start an engine with sensible
/// defaults.
pub type Engine = engine::Engine;
