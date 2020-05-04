use crate::error::Error;
use crate::plugin_manager::{PluginHandler, PluginManager};

type Result<T> = std::result::Result<T, Error>;

/// A convenient top-level engine type exposed to start an engine with sensible
/// defaults.
pub type DefaultEngine = Engine<PluginManager>;

#[derive(Default)]
pub struct Engine<T: PluginHandler> {
    plugin_manager: T,
}

impl<T: PluginHandler> Engine<T> {
    /// Register a new plugin with the engine.
    pub fn register_plugin(&mut self, path: &str) -> Result<()> {
        self.plugin_manager.register_plugin(path)?;

        Ok(())
    }

    /// Run the engine until completion.
    pub fn run(&mut self) -> Result<()> {
        println!("Hello, from engine!");

        self.plugin_manager.run_plugins()?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin_manager::Result;
    use std::collections::HashMap;

    mod register_plugin {
        use super::*;

        #[test]
        fn works() {
            let mut engine = mock_engine();
            engine.register_plugin("foo").unwrap();

            assert_eq!(engine.plugin_manager.plugins.get("foo"), Some(&0));
        }
    }

    mod run {
        use super::*;

        #[test]
        fn works() {
            let mut engine = mock_engine();
            engine.plugin_manager.plugins.insert("foo".to_owned(), 0);

            engine.run().unwrap();
            engine.run().unwrap();

            assert_eq!(engine.plugin_manager.plugins.get("foo"), Some(&2));
        }
    }

    fn mock_engine() -> Engine<MockPluginManager> {
        let plugin_manager = MockPluginManager::default();
        Engine { plugin_manager }
    }

    #[derive(Default)]
    struct MockPluginManager {
        plugins: HashMap<String, usize>,
    }

    impl PluginHandler for MockPluginManager {
        fn register_plugin(&mut self, path: &str) -> Result<()> {
            self.plugins.insert(path.to_owned(), 0);
            Ok(())
        }

        fn run_plugins(&mut self) -> Result<()> {
            for (_, val) in self.plugins.iter_mut() {
                *val += 1;
            }

            Ok(())
        }
    }
}
