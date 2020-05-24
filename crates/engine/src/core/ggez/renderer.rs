//! The renderer implementation for the ggez core.

use crate::{config, widget};
use common::{Color, Component, GameState, Shape};
use ggez::{graphics, nalgebra, Context, GameResult};
use std::time::Instant;

/// Handles rendering frames to the screen.
#[derive(Debug)]
pub struct Renderer {
    /// The configuration of the renderer.
    pub(crate) config: config::Renderer,

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
    pub fn run(
        &mut self,
        ctx: &mut Context,
        state: &GameState,
        _step_progress: f64,
    ) -> GameResult<()> {
        // Check if we are exceeding the configured max FPS
        if !self.should_render() {
            return Ok(());
        }

        // We're allowed to render. Record the timestamp for future render
        // decisions.
        self.last_step_timestamp = Instant::now();

        render_game_state(ctx, state)
    }

    /// Should the renderer render to the screen, based on the max FPS settings?
    fn should_render(&self) -> bool {
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
fn render_game_state(ctx: &mut Context, state: &GameState) -> GameResult<()> {
    graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

    for widget_with_position in state.widgets() {
        if !widget_with_position.is_visible() {
            continue;
        }

        // TODO: remove clone
        let widget = widget_with_position.widget().clone().into();
        let coordinates = widget_with_position.coordinates();

        for component in widget::components(&widget) {
            render_component(ctx, &component, coordinates);
        }
    }

    graphics::present(ctx)
}

/// Render a single component to the screen.
fn render_component(ctx: &mut Context, component: &Component, (mut x, mut y): (f32, f32)) {
    let (x_rel, y_rel) = component.coordinates;

    x += x_rel;
    y += y_rel;

    let drawable = match component.shape {
        Shape::Circle { radius, color } => graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            nalgebra::Point2::new(x, y),
            radius.max(1.0),
            2.0,
            into_color(color),
        ),
        Shape::Rectangle {
            width,
            height,
            color,
        } => graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            graphics::Rect {
                x,
                y,
                w: width,
                h: height,
            },
            into_color(color),
        ),
    };

    let result = drawable
        .and_then(|drawable| graphics::draw(ctx, &drawable, graphics::DrawParam::default()));

    if result.is_err() {
        todo!("logging")
    }
}

/// convert our color into a ggez color.
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
