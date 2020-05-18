use crate::config;
use common::{GameState, Value};
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

        Self::render_game_state(ctx, state)
    }

    fn render_game_state(ctx: &mut Context, state: &GameState) -> GameResult<()> {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        #[allow(clippy::cast_possible_truncation)]
        let pos_x = state
            .get("test")
            .and_then(|p| p.get("pos_x"))
            .and_then(Value::as_f64)
            .unwrap_or(0.0) as f32;

        #[allow(clippy::cast_possible_truncation)]
        let pos_y = state
            .get("test")
            .and_then(|p| p.get("pos_y"))
            .and_then(Value::as_f64)
            .unwrap_or(0.0) as f32;

        let circle = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            nalgebra::Point2::new(pos_x, pos_y),
            100.0,
            2.0,
            graphics::WHITE,
        )?;

        graphics::draw(ctx, &circle, (nalgebra::Point2::new(0.0, 0.0),))?;
        graphics::present(ctx)
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

impl From<config::Renderer> for Renderer {
    fn from(config: config::Renderer) -> Self {
        let minimum_nanoseconds_between_renders = match config.max_frames_per_second {
            Some(fps) => 1_000_000_000 / fps,
            None => 0,
        };

        Self {
            config,
            last_step_timestamp: Instant::now(),
            minimum_nanoseconds_between_renders,
        }
    }
}