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
use common::{event, Event, Key};
use once_cell::sync::OnceCell;
use std::{collections::HashSet, convert::TryInto};

// A horrible hack to make Coffee work with our current initialization set-up.
pub static mut BUILDER: OnceCell<Builder> = OnceCell::new();

// Run the engine.
//
// The `Engine` attribute does not contain anything useful, as it is constructed
// using `Engine::default()`. See the `build()` method on `EngineBuilder` to
// read more about why this is.
pub fn run(_: Engine) -> Result<(), Error> {
    let config = unsafe { BUILDER.get_unchecked() };
    let (width, height) = config.canvas.dimensions();

    let width = (width * 2).try_into().expect("window width too large");
    let height = (height * 2).try_into().expect("window height too large");

    let window = WindowSettings {
        title: "Vienna: work in progress".to_owned(),
        size: (width, height),
        resizable: false,
        fullscreen: false,
        maximized: false,
        vsync: config.vsync_enabled,
    };

    <Engine as Game>::run(window).unwrap();

    Ok(())
}

impl Game for Engine {
    const TICKS_PER_SECOND: u16 = 100;

    type Input = Keyboard;
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<Self> {
        let builder = unsafe { BUILDER.get_mut().unwrap() };
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
                KeyCode::Equals if input.is_key_pressed(KeyCode::LShift) => Key::Plus,
                KeyCode::Minus => Key::Minus,

                // modifier keys
                KeyCode::LShift | KeyCode::RShift => Key::Shift,
                KeyCode::LControl | KeyCode::RControl => Key::Ctrl,

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

        let event = Event::Input(event::Input::Keyboard { keys });
        if !self.updater.active_events.contains(&event) {
            self.updater.active_events.push(event);
        }
    }

    fn update(&mut self, _: &Window) {
        let canvas = self.config.canvas;
        let handler = self.plugin_handler.as_mut();

        self.updater
            .run(&mut self.game_state, canvas, handler)
            .unwrap();
    }

    fn draw(&mut self, frame: &mut Frame<'_>, _timer: &Timer) {
        self.renderer.run(frame, &self.game_state)
    }

    fn should_draw(&self) -> bool {
        self.renderer.should_run()
    }

    fn is_finished(&self) -> bool {
        self.updater.is_finished
    }
}
