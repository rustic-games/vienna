use crate::plugin::Handler;
use crate::{Builder, Error};

#[derive(Debug, Default)]
pub struct Engine<H: Handler> {
    pub(crate) plugin_handler: H,
}

impl<H: Handler> Engine<H> {
    pub fn builder<'a>() -> Builder<'a> {
        Builder::default()
    }
}

impl<H: Handler> Engine<H> {
    /// Run the engine until completion.
    pub fn run(&mut self) -> Result<(), Error> {
        println!("Hello, from engine!");

        self.plugin_handler
            .run_plugins()
            .map_err(anyhow::Error::new)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    mod run {
        use super::*;

        #[test]
        fn works() {
            let mut engine = mock_engine();
            engine.plugin_handler.plugins.insert("foo".to_owned(), 0);

            engine.run().unwrap();
            engine.run().unwrap();

            assert_eq!(engine.plugin_handler.plugins.get("foo"), Some(&2));
        }
    }

    fn mock_engine() -> Engine<MockHandler> {
        let plugin_handler = MockHandler::default();
        Engine { plugin_handler }
    }

    #[derive(Default)]
    struct MockHandler {
        plugins: HashMap<String, usize>,
    }

    impl Handler for MockHandler {
        type Error = std::ffi::NulError;

        fn register_plugin(&mut self, path: &str) -> Result<(), Self::Error> {
            self.plugins.insert(path.to_owned(), 0);
            Ok(())
        }

        fn run_plugins(&mut self) -> Result<(), Self::Error> {
            for (_, val) in self.plugins.iter_mut() {
                *val += 1;
            }

            Ok(())
        }
    }
}
