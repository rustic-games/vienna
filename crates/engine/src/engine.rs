use crate::{error, plugin::Handler, Builder, Error};

#[derive(Debug)]
pub struct Engine {
    pub(crate) plugin_handler: Box<dyn Handler>,
}

impl Engine {
    pub fn builder<'a>() -> Builder<'a> {
        Builder::default()
    }
}

impl Engine {
    /// Run the engine to completion.
    pub fn run(&mut self) -> Result<(), Error> {
        println!("Hello, from engine!");

        self.plugin_handler
            .run_plugins()
            .map_err(error::Runtime::from)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::mock;

    #[test]
    fn run() {
        let mut plugin_handler = mock::Manager::default();
        plugin_handler.register_plugin("").unwrap();

        let plugin_handler = Box::new(plugin_handler);
        let mut engine = Engine { plugin_handler };

        engine.run().unwrap();
        engine.run().unwrap();

        assert_eq!(engine.plugin_handler.as_mock().unwrap().plugins[0].runs, 2);
    }
}
