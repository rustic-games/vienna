use super::plugin::Plugin;
use crate::error;
use crate::plugin::{Handler, Runtime};
use common::GameState;
use std::path::Path;

/// A mock plugin implementation
#[derive(Debug, Default)]
pub struct Manager {
    pub(crate) plugins: Vec<Plugin>,
}

impl Handler for Manager {
    fn run_plugins(&mut self, game_state: &mut GameState) -> Result<(), error::Runtime> {
        for plugin in &mut self.plugins {
            plugin.run(game_state)?;
        }

        Ok(())
    }

    fn register_plugin(&mut self, _: &mut GameState, _: &Path) -> Result<(), error::Handler> {
        let plugin = Plugin::default();
        self.plugins.push(plugin);

        Ok(())
    }

    fn as_mock(&mut self) -> Option<&mut Self> {
        Some(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod run_plugins {
        use super::*;

        #[test]
        fn works() {
            let mut game_state = GameState::default();
            let mut manager = Manager::default();
            let plugin = Plugin::default();
            manager.plugins.push(plugin);

            assert!(manager.run_plugins(&mut game_state).is_ok())
        }
    }

    mod register_plugin {
        use super::*;

        #[test]
        fn works() {
            let mut state = GameState::default();
            let mut manager = Manager::default();
            manager.register_plugin(&mut state, Path::new("")).unwrap();
            manager.register_plugin(&mut state, Path::new("")).unwrap();

            assert_eq!(manager.plugins.len(), 2)
        }
    }
}
