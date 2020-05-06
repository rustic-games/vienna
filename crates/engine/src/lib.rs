#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

mod builder;
mod engine;
mod error;
mod plugin;

use builder::Builder;

pub use error::Error;

/// A convenient top-level engine type exposed to start an engine with sensible
/// defaults.
pub type Engine = engine::Engine<plugin::WasmManager>;
