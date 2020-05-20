pub(super) mod mock;
pub(super) mod wasm;

use crate::error;
use common::{Canvas, Event, GameState};
use core::fmt;
use displaydoc::Display;
use std::path::Path;

/// A list of exported functions the engine expects a plugin to have.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Display)]
pub enum Func {
    /// _init
    Init,

    /// _run
    Run,

    /// _malloc
    Malloc,
}

/// A runtime is configured to run all methods required for a plugin to be
/// usable by the engine.
pub trait Runtime {
    /// Run the plugin to completion.
    fn run(
        &mut self,
        game_state: &mut GameState,
        canvas: Canvas,
        events: &[Event],
    ) -> Result<(), error::Runtime>;

    /// The name of the plugin.
    fn name(&self) -> &str;

    /// Get the concrete `wasm::Plugin` implementation, if the underlying type
    /// matches.
    fn as_wasm(&mut self) -> Option<&mut wasm::Plugin> {
        None
    }

    /// Get the concrete `mock::Plugin` implementation, if the underlying type
    /// matches.
    fn as_mock(&mut self) -> Option<&mut mock::Plugin> {
        None
    }
}

/// A handler takes ownership of external plugins, and runs them when requested.
pub trait Handler {
    /// Run all registered plugins.
    fn run_plugins(
        &mut self,
        game_state: &mut GameState,
        canvas: Canvas,
        events: &[Event],
    ) -> Result<(), error::Runtime>;

    /// Register a new plugin to handle.
    fn register_plugin(
        &mut self,
        game_state: &mut GameState,
        file: &Path,
    ) -> Result<(), error::Handler>;

    /// Get the concrete `wasm::Manager` implementation, if the underlying type
    /// matches.
    fn as_wasm(&mut self) -> Option<&mut wasm::Manager> {
        None
    }

    /// Get the concrete `mock::Manager` implementation, if the underlying type
    /// matches.
    fn as_mock(&mut self) -> Option<&mut mock::Manager> {
        None
    }
}

impl fmt::Debug for dyn Handler {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("dyn Handler")
    }
}
