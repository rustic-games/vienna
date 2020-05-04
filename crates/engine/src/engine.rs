use crate::error::Error;
use crate::plugin_manager::PluginManager;

type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
pub struct Engine {
    plugin_manager: PluginManager,
}

impl Engine {
    pub fn register_plugin(&mut self, path: &str) -> Result<()> {
        self.plugin_manager.register_plugin(path)?;

        Ok(())
    }

    pub fn run(&mut self) -> Result<()> {
        println!("Hello, from engine!");

        self.plugin_manager.run_plugins()?;

        Ok(())
    }
}
