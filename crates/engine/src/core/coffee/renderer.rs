use crate::config;
use coffee::graphics::{Color, Frame, Mesh, Point, Shape};
use common::{GameState, Value};
use std::time::Instant;

#[derive(Debug)]
pub struct Renderer {
    config: config::Renderer,
    last_step_timestamp: Instant,
    minimum_nanoseconds_between_renders: u64,
}

impl Renderer {
    /// Render the state of the game to the screen.
    pub fn run(&mut self, frame: &mut Frame, state: &GameState) {
        // We're allowed to render. Record the timestamp for future render
        // decisions.
        self.last_step_timestamp = Instant::now();

        Self::render_game_state(frame, state)
    }

    /// Should the renderer render to the screen, based on the max FPS settings?
    pub fn should_run(&self) -> bool {
        if self.minimum_nanoseconds_between_renders == 0 {
            return true;
        }

        let last_step_duration = self.last_step_timestamp.elapsed();

        #[allow(clippy::cast_possible_truncation)]
        let last_step_nanoseconds = last_step_duration.as_nanos() as u64;

        last_step_nanoseconds >= self.minimum_nanoseconds_between_renders
    }

    fn render_game_state(frame: &mut Frame, state: &GameState) {
        frame.clear(Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        });

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

        let shape = Shape::Circle {
            center: Point::new(pos_x * 4.0, pos_y * 4.0),
            radius: 200.0,
        };

        let mut mesh = Mesh::new();
        mesh.fill(shape, Color::WHITE);
        mesh.draw(&mut frame.as_target());
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
