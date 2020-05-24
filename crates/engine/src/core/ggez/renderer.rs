use crate::{config, widget};
use common::{Color, Component, GameState, Shape};
use ggez::{graphics, nalgebra, Context, GameResult};
use std::time::Instant;

#[derive(Debug)]
pub struct Renderer {
    pub(crate) config: config::Renderer,
    last_step_timestamp: Instant,
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

        #[allow(clippy::cast_possible_truncation)]
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
        _ => todo!(),
    };

    graphics::draw(
        ctx,
        &drawable.expect("TODO"),
        graphics::DrawParam::default(),
    )
    .expect("TODO");
}

fn into_color(color: Color) -> graphics::Color {
    let Color { r, g, b, a } = color;
    graphics::Color { r, g, b, a }
}

impl From<config::Renderer> for Renderer {
    fn from(config: config::Renderer) -> Self {
        let minimum_nanoseconds_between_renders = match config.max_frames_per_second {
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
