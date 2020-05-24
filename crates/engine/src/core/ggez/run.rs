//! An abstraction over the "ggez" game engine.
//!
//! Only this module explicitly depends on `ggez` types, in theory.
//!
//! In practice the renderer still contains implementation details to the used
//! game engine, but this will be removed once plugins are in control of drawing
//! to the screen.

use crate::{error, Engine, Error};
use common::{event, Event, Key};
use ggez::{
    conf::{FullscreenType, ModuleConf, NumSamples, WindowMode, WindowSetup},
    event::EventHandler,
    input::keyboard::{self, KeyCode, KeyMods},
    Context, ContextBuilder, GameResult,
};
use std::{collections::HashSet, path::Path};

#[allow(clippy::cast_precision_loss)]
pub fn run(mut engine: Engine) -> Result<(), Error> {
    let window_setup = WindowSetup {
        title: "Vienna: work in progress".to_owned(),
        samples: NumSamples::Zero,
        vsync: true,
        icon: "".to_owned(),
        srgb: true,
    };

    let (width, height) = engine.config.canvas.dimensions();

    let window_mode = WindowMode {
        width: f32::from(width),
        height: f32::from(height),
        maximized: false,
        fullscreen_type: FullscreenType::Windowed,
        borderless: false,
        min_width: 0.0,
        max_width: 0.0,
        min_height: 0.0,
        max_height: 0.0,
        resizable: false,
    };

    let modules = ModuleConf {
        gamepad: false,
        audio: true,
    };

    let (mut ctx, mut event_loop) = ContextBuilder::new("Vienna", "")
        .window_setup(window_setup)
        .window_mode(window_mode)
        .modules(modules)
        .with_conf_file(false)
        .add_resource_path(Path::new("./resources"))
        .build()
        .unwrap();

    ggez::event::run(&mut ctx, &mut event_loop, &mut engine).map_err(Into::into)
}

impl EventHandler for Engine {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut keys = HashSet::new();
        for pressed_key in keyboard::pressed_keys(ctx) {
            let key = match pressed_key {
                // letter keys
                KeyCode::A => Key::A,
                KeyCode::B => Key::B,
                KeyCode::D => Key::D,
                KeyCode::E => Key::E,
                KeyCode::G => Key::G,
                KeyCode::Q => Key::Q,
                KeyCode::R => Key::R,
                KeyCode::S => Key::S,
                KeyCode::W => Key::W,

                // other keys
                KeyCode::Equals if keyboard::is_mod_active(ctx, KeyMods::SHIFT) => Key::Plus,
                KeyCode::Minus => Key::Minus,

                // modifier keys
                KeyCode::LShift | KeyCode::RShift => Key::Shift,
                KeyCode::LControl | KeyCode::RControl => Key::Ctrl,

                // All other keys are ignored for now.
                _ => return Ok(()),
            };

            keys.insert(key);
        }

        let mut events = vec![];
        if !keys.is_empty() {
            events.push(Event::Input(event::Input::Keyboard { keys }));
        }

        let canvas = self.config.canvas;
        let handler = self.plugin_handler.as_mut();
        self.updater
            .run(&mut self.game_state, canvas, &events, handler)
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

        // TODO: For now the renderer is not engine-agnostic, but will be once
        //       plugins are in charge of drawing to the screen.
        self.renderer.run(ctx, &self.game_state, progress)
    }
}
