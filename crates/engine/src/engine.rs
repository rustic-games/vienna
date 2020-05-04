use crate::error::Error;
use crate::plugin_manager::PluginManager;

type Result<T> = std::result::Result<T, Error>;

#[derive(Default)]
pub struct Engine {
    plugin_manager: PluginManager,
}

impl Engine {
    pub fn run(&mut self) -> Result<()> {
        println!("Hello, from engine!");

        let test = "tests/fixtures/test.wat";
        self.plugin_manager.register_plugin(test)?;
        self.plugin_manager.run_plugins()?;

        Ok(())
    }
}
