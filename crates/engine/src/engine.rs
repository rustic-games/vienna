use crate::{error, plugin::Handler, Builder, Error};

#[derive(Debug)]
pub struct Engine {
    pub(crate) plugin_handler: Box<dyn Handler>,
    pub(crate) continuous: bool,
}

impl Engine {
    pub fn builder<'a>() -> Builder<'a> {
        Builder::default()
    }
}

impl Engine {
    /// Run the engine to completion.
    pub fn run(&mut self) -> Result<(), Error> {
        use std::thread::sleep;
        use std::time::{Duration, Instant};

        println!("Hello, from engine!");

        let mut total_duration = Duration::default();
        let mut timer = Instant::now();
        let mut count = 0;

        loop {
            count += 1;
            let tick_duration = timer.elapsed();
            total_duration += tick_duration;

            println!(
                "game tick | count={:<3}  duration_since_previous_tick={:.3?}    duration_total={:.3?}",
                count, tick_duration, total_duration
            );

            timer = Instant::now();

            self.plugin_handler
                .run_plugins()
                .map_err(error::Runtime::from)?;

            if !self.continuous {
                break;
            }

            sleep(Duration::new(1, 0))
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::plugin::mock;

    #[test]
    fn run() {
        let continuous = false;
        let mut plugin_handler = mock::Manager::default();
        plugin_handler.register_plugin("").unwrap();

        let plugin_handler = Box::new(plugin_handler);
        let mut engine = Engine {
            plugin_handler,
            continuous,
        };

        engine.run().unwrap();
        engine.run().unwrap();

        assert_eq!(engine.plugin_handler.as_mock().unwrap().plugins[0].runs, 2);
    }
}
