#![warn(clippy::all)]
#![warn(clippy::pedantic)]

mod builder;
mod engine;
mod error;
mod plugin;
mod plugin_manager;

use crate::builder::Builder;
use error::Error;
use plugin_manager::PluginManager;

/// A convenient top-level engine type exposed to start an engine with sensible
/// defaults.
pub type Engine = engine::Engine<PluginManager<plugin::Wasm>>;
