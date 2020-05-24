//! A mock plugin implementation.

use crate::error;
use crate::plugin::Runtime;
use common::{Canvas, Event, GameState};

/// A mock plugin implementation
#[derive(Debug, Default)]
pub struct Plugin {
    /// The amount of times this plugin "ran" (mocked).
    pub(crate) runs: usize,

    /// The state of the game.
    pub(crate) game_state: GameState,
}

impl Runtime for Plugin {
    fn run(&mut self, _: &mut GameState, _: Canvas, _: &[Event]) -> Result<(), error::Runtime> {
        self.runs = self.runs.saturating_add(1);

        Ok(())
    }

    fn name(&self) -> &str {
        ""
    }

    fn as_mock(&mut self) -> Option<&mut Self> {
        Some(self)
    }
}

#[cfg(test)]
#[allow(clippy::restriction)]
mod tests {
    use super::*;

    #[test]
    fn run() {
        let canvas = Canvas::default();
        let mut mock = Plugin::default();
        let mut game_state = GameState::default();
        mock.run(&mut game_state, canvas, &[]).unwrap();
        mock.run(&mut game_state, canvas, &[]).unwrap();

        assert_eq!(mock.runs, 2)
    }

    #[test]
    fn name() {
        assert_eq!(Plugin::default().name(), "")
    }
}
