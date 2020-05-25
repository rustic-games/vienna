//! An abstraction over the "coffee" game engine.
//!
//! Only this module explicitly depends on `coffee` types, in theory.

use crate::{error, Builder, Engine, Error};
use coffee::{
    graphics::{Frame, Window, WindowSettings},
    input::{keyboard::KeyCode, mouse::Button, KeyboardAndMouse},
    load::Task,
    Game, Timer,
};
use common::{event, Event, Key};
use once_cell::sync::OnceCell;
use std::{collections::HashSet, convert::TryInto};

/// A horrible hack to make Coffee work with our current initialization set-up.
pub static mut BUILDER: OnceCell<Builder> = OnceCell::new();

/// Run the engine.
///
/// The `Engine` attribute does not contain anything useful, as it is constructed
/// using `Engine::default()`. See the `build()` method on `EngineBuilder` to
/// read more about why this is.
pub fn run(_: Engine) -> Result<(), Error> {
    let config = unsafe { BUILDER.get_unchecked() };
    let (width, height) = config.canvas.dimensions();

    let width = (width.saturating_mul(2))
        .try_into()
        .map_err(|_| error::Builder::WindowSize(width))?;

    let height = (height.saturating_mul(2))
        .try_into()
        .map_err(|_| error::Builder::WindowSize(height))?;

    let window = WindowSettings {
        title: "Vienna: work in progress".to_owned(),
        size: (width, height),
        resizable: false,
        fullscreen: false,
        maximized: false,
        vsync: config.vsync_enabled,
    };

    <Engine as Game>::run(window).map_err(Into::into)
}

impl Game for Engine {
    const TICKS_PER_SECOND: u16 = 100;

    type Input = KeyboardAndMouse;
    type LoadingScreen = (); // No loading screen

    fn load(_window: &Window) -> Task<Self> {
        let builder = match unsafe { BUILDER.get_mut() } {
            Some(builder) => builder,
            None => todo!("logging"),
        };

        #[allow(clippy::match_wild_err_arm)]
        let engine = match builder.build_inner() {
            Ok(engine) => engine,
            Err(_) => todo!("logging"),
        };

        Task::succeed(|| engine)
    }

    fn interact(&mut self, input: &mut Self::Input, _: &mut Window) {
        let mut events = vec![];

        // Handle cursor input if needed.
        if input.mouse().is_cursor_within_window() {
            // mouse position
            let position = input.mouse().cursor_position();

            // divided by two, because of Coffee's issue with high-DPI (see
            // documentation for `render_component()`).
            let (x, y) = (position.x / 2.0, position.y / 2.0);

            let event = Event::Input(event::Input::Pointer(x, y));
            events.push(event);

            for button in &[Button::Left, Button::Middle, Button::Right] {
                for point in input.mouse().button_clicks(*button) {
                    let button = convert_button(button);
                    let event = Event::Input(event::Input::MouseClick {
                        button,
                        x: point.x / 2.0,
                        y: point.y / 2.0,
                    });

                    events.push(event);
                }
            }
        }

        if !input.keyboard().pressed_keys().is_empty() {
            let mut keys = HashSet::new();
            for pressed_key in input.keyboard().pressed_keys() {
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
                    KeyCode::Equals if input.keyboard().is_key_pressed(KeyCode::LShift) => {
                        Key::Plus
                    }
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
                    _ => break,
                };

                keys.insert(key);
            }

            let event = Event::Input(event::Input::Keyboard { keys });
            events.push(event)
        }

        for event in events {
            if !self.updater.active_events.contains(&event) {
                self.updater.active_events.push(event);
            }
        }
    }

    fn update(&mut self, _: &Window) {
        let canvas = self.config.canvas;
        let handler = self.plugin_handler.as_mut();

        let result = self.updater.run(&mut self.game_state, canvas, handler);

        if result.is_err() {
            todo!("logging")
        }
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

fn convert_button(button: &coffee::input::mouse::Button) -> event::MouseButton {
    match button {
        Button::Left => event::MouseButton::Left,
        Button::Middle => event::MouseButton::Middle,
        Button::Right => event::MouseButton::Right,
        _ => event::MouseButton::Other,
    }
}
