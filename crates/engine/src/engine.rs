use crate::{
    config,
    core::{Renderer, Updater},
    plugin::Handler,
    Builder, Error,
};
use common::GameState;

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
    pub fn builder() -> Builder {
        Builder::default()
    }

    pub fn run(self) -> Result<(), Error> {
        crate::core::run(self)
    }
}
