use crate::{
    event, widget, Color, Component, Deserialize, Event, Key, Serialize, Shape, Value, WidgetState,
};
use std::{
    collections::{HashMap, HashSet},
    convert::TryFrom,
};

/// An example widget of an interactive circle.
///
/// - The circle triggers the "move" event whenever the `WASD` keys are used by
///   the player.
///
/// - The `Shift` and `Ctrl` modifier keys modify the "move" event to add
///   details about the requested movement speed.
///
/// - The `Q` and `E` keys trigger the "resized" event and modify the circle's
///   radius.
///
/// - The `R`, `G` and `B` keys modify the circle's color.
///
/// - The `-` and `+` keys modify the circle's opacity.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct MovingCircle {
    radius: f32,
    color: Color,

    // keeps track of whether or not the color shifting is going up or down.
    //
    // This allows a single key to continuously shift the color space without
    // any jarring jumps from high to low at the boundaries.
    r_up: bool,
    g_up: bool,
    b_up: bool,
}

/// The direction in which the widget wants to be moved by its owner, based on
/// the incoming key events.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

/// The speed at which the widget wants to be moved by its owner, based on the
/// incoming key events.
#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
enum Speed {
    Normal,
    Fast,
    Turbo,
}

impl MovingCircle {
    /// Resize the circle based on the provided key.
    fn resize(&mut self, step: f32, key: Key) -> Option<event::Widget> {
        match key {
            Key::Q => self.radius = 0.0_f32.max(self.radius - step),
            Key::E => self.radius = std::f32::MAX.min(self.radius + step),
            _ => return None,
        };

        Some(event::Widget::new("resized"))
    }

    /// Shift the circle color based on the provided key.
    fn shift_color(&mut self, mut step: f32, key: Key) -> Option<event::Widget> {
        let (up, color) = match key {
            Key::R => (&mut self.r_up, &mut self.color.r),
            Key::G => (&mut self.g_up, &mut self.color.g),
            Key::B => (&mut self.b_up, &mut self.color.b),
            _ => return None,
        };

        // Depending on the "up" toggle, we move up or down the color spectrum.
        if !*up {
            step *= -1.0;
        }

        let mut new_color = *color + step;

        // If we've reach the end of the color spectrum, we switch the key to
        // move the spectrum down again.
        if new_color > 1.0 {
            *up = false;
            new_color = 1.0;
        }

        // Similar to above, but this time for the lowest end of the color
        // spectrum, switching the key to move up again.
        if new_color < 0.0 {
            *up = true;
            new_color = 0.0;
        }

        *color = new_color;

        None
    }

    /// Shift the circle alpha based on the provided key.
    fn shift_alpha(&mut self, step: f32, key: Key) -> Option<event::Widget> {
        match key {
            Key::Plus => self.color.a = 1.0_f32.min(self.color.a + step),
            Key::Minus => self.color.a = 0.0_f32.max(self.color.a - step),
            _ => {}
        };

        None
    }
}

impl widget::Runtime for MovingCircle {
    fn attribute(&self, key: &str) -> Option<Value> {
        match key {
            "radius" => Some(self.radius.into()),
            "color" => Some(serde_json::to_value(self.color).expect("TODO")),
            _ => None,
        }
    }

    fn attribute_mut(&mut self, key: &str, cb: fn(value: Option<&mut Value>)) {
        match key {
            "radius" => {
                let mut value = Value::from(self.radius);
                cb(Some(&mut value));

                #[allow(clippy::cast_possible_truncation)]
                let radius = value.as_f64().expect("TODO") as f32;
                self.radius = radius;
            }
            "color" => {
                let mut value = serde_json::to_value(self.color).ok();
                cb(value.as_mut());

                if let Some(value) = value {
                    self.color = serde_json::from_value(value).expect("TODO");
                }
            }
            _ => cb(None),
        }
    }

    fn dimensions(&self) -> (f32, f32) {
        let diameter = self.radius * 2.0;

        (diameter, diameter)
    }

    fn state(&self) -> WidgetState {
        let mut state = HashMap::with_capacity(5);

        state.insert("radius", self.radius.into());
        state.insert("color", self.color.into());
        state.insert("r_up", self.r_up.into());
        state.insert("g_up", self.g_up.into());
        state.insert("b_up", self.b_up.into());

        WidgetState::new(widget::Kind::MovingCircle, state)
    }

    fn interact(&mut self, event: &Event) -> Vec<event::Widget> {
        let mut output = vec![];

        if let Event::Input(event::Input::Keyboard { keys }) = event {
            for key in keys {
                let event = match key {
                    Key::W | Key::A | Key::S | Key::D => move_event(*key, keys),
                    Key::Q | Key::E => self.resize(1.0, *key),
                    Key::R | Key::G | Key::B => self.shift_color(0.01, *key),
                    Key::Plus | Key::Minus => self.shift_alpha(0.01, *key),
                    _ => None,
                };

                if let Some(event) = event {
                    output.push(event);
                }
            }
        };

        output
    }

    fn render(&self) -> Vec<Component> {
        let shape = Shape::Circle {
            radius: self.radius,
            color: self.color,
        };

        let component = Component {
            shape,
            coordinates: (0.0, 0.0),
        };

        vec![component]
    }
}

/// Generate the "move" event based on the provided key and modifiers.
///
/// A widget does not control its own location on the canvas, so it has to ask
/// its owner to change its location.
fn move_event(key: Key, modifiers: &HashSet<Key>) -> Option<event::Widget> {
    let speed = match () {
        _ if modifiers.contains(&Key::Shift) => Speed::Fast,
        _ if modifiers.contains(&Key::Ctrl) => Speed::Turbo,
        _ => Speed::Normal,
    };

    let direction = match key {
        Key::W => Direction::Up,
        Key::S => Direction::Down,
        Key::A => Direction::Left,
        Key::D => Direction::Right,
        _ => return None,
    };

    let mut event = event::Widget::new("move");
    event.add_attribute("direction", direction);
    event.add_attribute("speed", speed);

    Some(event)
}

impl TryFrom<&WidgetState> for MovingCircle {
    type Error = String;

    fn try_from(state: &WidgetState) -> Result<Self, Self::Error> {
        #[allow(clippy::cast_possible_truncation)]
        let radius = state
            .get("radius")
            .ok_or("missing `radius` attribute")?
            .as_f64()
            .ok_or("`radius` must be a number")? as f32;

        let color = state
            .get("color")
            .ok_or("missing `color` attribute")?
            .clone();

        let color = serde_json::from_value(color).map_err(|_| "invalid `color` attribute")?;

        let r_up = state.get("r_up").and_then(Value::as_bool).unwrap_or(true);
        let g_up = state.get("g_up").and_then(Value::as_bool).unwrap_or(true);
        let b_up = state.get("b_up").and_then(Value::as_bool).unwrap_or(true);

        Ok(Self {
            radius,
            color,
            r_up,
            g_up,
            b_up,
        })
    }
}