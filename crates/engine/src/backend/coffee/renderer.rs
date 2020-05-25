//! The renderer implementation for the coffee backend.

use crate::{config, widget};
use coffee::graphics::{self, Frame, Mesh, Point};
use common::{Color, Component, GameState, Shape};
use std::time::Instant;

/// Handles rendering frames to the screen.
#[derive(Debug)]
pub struct Renderer {
    /// The configuration of the renderer.
    config: config::Renderer,

    /// A cache of the timestamp the last step finished.
    ///
    /// This is used to adhere to any configured FPS limits.
    last_step_timestamp: Instant,

    /// A cache based off the FPS configuration.
    ///
    /// This is used to adhere to any configured FPS limits.
    minimum_nanoseconds_between_renders: u64,
}

impl Renderer {
    /// Render the state of the game to the screen.
    pub fn run(&mut self, frame: &mut Frame<'_>, state: &GameState) {
        // We're allowed to render. Record the timestamp for future render
        // decisions.
        self.last_step_timestamp = Instant::now();

        render_game_state(frame, state)
    }

    /// Should the renderer render to the screen, based on the max FPS settings?
    pub fn should_run(&self) -> bool {
        if self.minimum_nanoseconds_between_renders == 0 {
            return true;
        }

        let last_step_duration = self.last_step_timestamp.elapsed();

        #[allow(clippy::cast_possible_truncation, clippy::as_conversions)]
        let last_step_nanoseconds = last_step_duration.as_nanos() as u64;

        last_step_nanoseconds >= self.minimum_nanoseconds_between_renders
    }
}

/// Render the state of the game to the screen.
fn render_game_state(frame: &mut Frame<'_>, state: &GameState) {
    frame.clear(graphics::Color {
        r: 0.1,
        g: 0.2,
        b: 0.3,
        a: 1.0,
    });

    for widget_with_position in state.widgets() {
        if !widget_with_position.is_visible() {
            continue;
        }

        // TODO: remove clone
        let state = widget_with_position.state().clone().into();
        let coordinates = widget_with_position.coordinates();

        for component in widget::components(&state) {
            render_component(frame, &component, coordinates);
        }
    }
}

/// The Coffee backend does not support high-DPI mode yet (retina screens).
///
/// See: <https://github.com/hecrj/coffee/issues/6>
///
/// The current way to deal with this works as follows:
///
/// When building the engine, the width and height of the window are defined by
/// the `game_state`.
///
/// When the `coffee` backend is used, these values will be doubled so that the
/// window is the correct size on retina screens.
///
/// The actual size of the "canvas" is left unchanged. This allows the plugins
/// to use "points" as if they are pixels.
///
/// Then, in this function when we convert a widget to an actual graphic, we
/// double all pixel values.
fn render_component(frame: &mut Frame<'_>, component: &Component, (mut x, mut y): (f32, f32)) {
    let (x_rel, y_rel) = component.coordinates;

    x += x_rel;
    y += y_rel;

    let mesh = match component.shape {
        Shape::Circle {
            radius,
            fill,
            border,
        } => {
            let shape = graphics::Shape::Circle {
                center: Point::new((x + radius) * 2.0, (y + radius) * 2.0),
                radius: radius * 2.0,
            };

            let mut mesh = Mesh::new();
            mesh.fill(shape, into_color(fill));

            if let Some(border) = border {
                // Make sure the border falls inside the circle's radius.
                let border_radius = radius - border.width / 4.0;

                let shape = graphics::Shape::Circle {
                    center: Point::new((x + radius) * 2.0, (y + radius) * 2.0),
                    radius: border_radius * 2.0,
                };

                mesh.stroke(shape, into_color(border.color), border.width);
            }

            mesh
        }

        Shape::Rectangle {
            width,
            height,
            color,
        } => {
            let rect = graphics::Rectangle {
                x,
                y,
                width,
                height,
            };

            let shape = graphics::Shape::Rectangle(rect);

            let mut mesh = Mesh::new();
            mesh.fill(shape, into_color(color));
            mesh
        }
    };

    mesh.draw(&mut frame.as_target());
}

/// Convert our color struct to Coffee's one.
const fn into_color(color: Color) -> graphics::Color {
    let Color { r, g, b, a } = color;
    graphics::Color { r, g, b, a }
}

impl From<config::Renderer> for Renderer {
    fn from(config: config::Renderer) -> Self {
        let minimum_nanoseconds_between_renders = match config.max_frames_per_second {
            #[allow(clippy::integer_division, clippy::integer_arithmetic)]
            Some(fps) => 1_000_000_000 / u64::from(fps),
            None => 0,
        };

        Self {
            config,
            last_step_timestamp: Instant::now(),
            minimum_nanoseconds_between_renders,
        }
    }
}
