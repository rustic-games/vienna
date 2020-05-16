use crate::error;
use crate::plugin::Runtime;
use common::GameState;

/// A mock plugin implementation
#[derive(Debug, Default)]
pub struct Plugin {
    pub(crate) runs: usize,
    pub(crate) game_state: GameState,
}

impl Runtime for Plugin {
    fn run(&mut self, _: &mut GameState) -> Result<(), error::Runtime> {
        self.runs += 1;

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
mod tests {
    use super::*;

    #[test]
    fn run() {
        let mut mock = Plugin::default();
        let mut game_state = GameState::default();
        mock.run(&mut game_state).unwrap();
        mock.run(&mut game_state).unwrap();

        assert_eq!(mock.runs, 2)
    }

    #[test]
    fn name() {
        assert_eq!(Plugin::default().name(), "")
    }
}
