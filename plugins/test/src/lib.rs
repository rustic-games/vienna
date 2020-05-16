vienna::load!();

fn init() -> Registration {
    Registration::new("test")
        .state("pos_y", Value::from(0.0))
        .state("pos_x", Value::from(0.0))
}

fn run(sdk: &mut Sdk) -> Result<()> {
    let mut plugin = Plugin { sdk };
    plugin.handle_events();

    Ok(())
}

struct Plugin<'a> {
    sdk: &'a mut Sdk,
}

impl<'a> Plugin<'a> {
    fn handle_events(&mut self) {
        let events = self.sdk.events().to_vec();

        for event in events {
            if let Event::Keyboard(keys) = event {
                keys.into_iter().for_each(|k| self.handle_key(k));
            }
        }
    }

    fn handle_key(&mut self, key: Key) {
        match key {
            Key::W | Key::S => {
                let value_y = self.sdk.get_mut("pos_y").unwrap();
                let pos_y = value_y.as_f64().unwrap_or(0.0) as f32;

                match key {
                    Key::W if pos_y > 0.0 => *value_y = Value::from(pos_y - 1.0),
                    Key::S if pos_y < 600.0 => *value_y = Value::from(pos_y + 1.0),
                    _ => {}
                }
            }
            Key::A | Key::D => {
                let value_x = self.sdk.get_mut("pos_x").unwrap();
                let pos_x = value_x.as_f64().unwrap_or(0.0) as f32;

                match key {
                    Key::A if pos_x > 0.0 => *value_x = Value::from(pos_x - 1.0),
                    Key::D if pos_x < 800.0 => *value_x = Value::from(pos_x + 1.0),
                    _ => {}
                }
            }
        }
    }
}
