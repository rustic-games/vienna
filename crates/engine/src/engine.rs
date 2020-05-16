use crate::{config, error, plugin::Handler, Builder, Error, Renderer, Updater};
use common::GameState;
use ggez::{event::EventHandler, Context, GameResult};
use std::path::Path;

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
    pub fn builder<'a>() -> Builder<'a> {
        Builder::default()
    }

    pub fn run(mut self) -> Result<(), Error> {
        use ggez::conf::{ModuleConf, NumSamples, WindowSetup};
        use ggez::{event, ContextBuilder};

        let window_setup = WindowSetup {
            title: "Vienna: work in progress".to_owned(),
            samples: NumSamples::Zero,
            vsync: true,
            icon: "".to_owned(),
            srgb: true,
        };

        let modules = ModuleConf {
            gamepad: false,
            audio: true,
        };

        let (mut ctx, mut event_loop) = ContextBuilder::new("Vienna", "")
            .window_setup(window_setup)
            .modules(modules)
            .with_conf_file(false)
            .add_resource_path(Path::new("./resources"))
            .build()
            .unwrap();

        event::run(&mut ctx, &mut event_loop, &mut self).map_err(Into::into)
    }
}

impl EventHandler for Engine {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let handler = self.plugin_handler.as_mut();

        self.updater
            .run(&mut self.game_state, handler)
            .map_err(|err| match err {
                // this is the only native error type supported by ggez
                error::Updater::GameEngine(err) => err,

                // any other errors can't be propagated in a nice way, so we'll
                // make due with what we have.
                error::Updater::PluginRuntime(err) => {
                    ggez::GameError::RenderError(format!("{:#}", anyhow::Error::new(err)))
                }
            })
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let progress = self.updater.step_progress;

        self.renderer.run(ctx, &self.game_state, progress)
    }
}
