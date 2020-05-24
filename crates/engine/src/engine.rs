//! The main engine implementation.

use crate::{
    backend::{Renderer, Updater},
    config,
    plugin::Handler,
    Builder, Error,
};
use common::GameState;

/// The top-level object that holds all the configuration, state, and logic.
#[derive(Debug)]
pub struct Engine {
    /// The global engine configuration.
    pub(super) config: config::Engine,

    /// The updater of the engine.
    pub(super) updater: Updater,

    /// The renderer of the engine.
    pub(super) renderer: Renderer,

    /// The state of the game.
    pub(super) game_state: GameState,

    /// The plugin store.
    pub(super) plugin_handler: Box<dyn Handler>,
}

impl Default for Engine {
    fn default() -> Self {
        let plugin_handler = Box::new(crate::plugin::wasm::Manager::default());

        Self {
            config: config::Engine::default(),
            updater: config::Updater::default().into(),
            renderer: config::Renderer::default().into(),
            game_state: GameState::default(),
            plugin_handler,
        }
    }
}

impl Engine {
    /// Get a new builder to create a new engine instance.
    pub fn builder() -> Builder {
        Builder::default()
    }

    /// Run the engine to completion or until an error occurs.
    pub fn run(self) -> Result<(), Error> {
        crate::backend::run(self)
    }
}
