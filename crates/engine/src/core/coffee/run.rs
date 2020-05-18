//! An abstraction over the "coffee" game engine.
//!
//! Only this module explicitly depends on `coffee` types, in theory.

use crate::{Builder, Engine, Error};
use coffee::{
    graphics::{Frame, Window, WindowSettings},
    input::keyboard::{KeyCode, Keyboard},
    load::Task,
    Game, Timer,
};
use common::{Event, GameState, Key};
use once_cell::sync::OnceCell;
use std::collections::HashSet;

// A horrible hack to make Coffee work with our current initialization set-up.
pub static mut CONFIG: OnceCell<Config> = OnceCell::new();

#[derive(Debug)]
pub struct Config {
    plugin_paths: Vec<String>,
    game_state: Option<GameState>,
}

impl Config {
    pub fn new(plugin_paths: Vec<String>, game_state: Option<GameState>) -> Self {
        Self {
            plugin_paths,
            game_state,
        }
    }
}

pub fn run(_: Engine) -> Result<(), Error> {
    let window = WindowSettings {
        title: "Vienna: work in progress".to_owned(),
        // retina: https://github.com/hecrj/coffee/issues/6
        size: (1600, 1200),
        resizable: false,
        fullscreen: false,
        maximized: false,
        vsync: false,
    };

    <Engine as Game>::run(window).unwrap();

    Ok(())
}

impl Game for Engine {
    const TICKS_PER_SECOND: u16 = 100;

    type Input = Keyboard;
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<Self> {
        let config = unsafe { CONFIG.get_mut().unwrap() };

        let mut builder = Builder::default();

        for path in &config.plugin_paths {
            builder = builder.with_plugin_path(path);
        }

        builder = builder.with_game_state(config.game_state.take().unwrap_or_default());

        let engine = builder.build_inner().unwrap();

        Task::succeed(|| engine)
    }

    fn interact(&mut self, input: &mut Self::Input, _: &mut Window) {
        if input.pressed_keys().is_empty() {
            return;
        }

        let mut keys = HashSet::new();
        for pressed_key in input.pressed_keys() {
            let key = match pressed_key {
                KeyCode::W => Key::W,
                KeyCode::A => Key::A,
                KeyCode::S => Key::S,
                KeyCode::D => Key::D,

                // Quit engine.
                KeyCode::Escape => {
                    self.updater.is_finished = true;
                    return;
                }

                // All other keys are ignored for now.
                _ => return,
            };

            keys.insert(key);
        }

        let event = Event::Keyboard(keys);
        if !self.updater.active_events.contains(&event) {
            self.updater.active_events.push(event);
        }
    }

    fn update(&mut self, _: &Window) {
        let handler = self.plugin_handler.as_mut();

        self.updater.run(&mut self.game_state, handler).unwrap();
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        self.renderer.run(frame, &self.game_state)
    }

    fn should_draw(&self) -> bool {
        self.renderer.should_run()
    }

    fn is_finished(&self) -> bool {
        self.updater.is_finished
    }
}
