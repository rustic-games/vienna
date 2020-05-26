//! A rectangular button.

use crate::{
    event, widget, Color, Component, Deserialize, Event, Serialize, Shape, Value, WidgetState,
};
use std::{collections::HashMap, convert::TryFrom};

/// A rectangular button.
///
/// This widget renders a button with hover/active states, a button text, and
/// emits the "activated" event when the button is clicked.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ButtonRectangle {
    /// The width of the button.
    width: f32,

    /// The height of the button.
    height: f32,

    /// The color of the button in idle state.
    idle_color: Color,

    /// The color of the button in focus state.
    focus_color: Color,

    /// The color of the button in active state.
    active_color: Color,

    /// The state of the button.
    state: ButtonState,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[allow(clippy::missing_docs_in_private_items)]
enum ButtonState {
    Idle,
    Focus,
    Active,
}

impl TryFrom<&WidgetState> for ButtonRectangle {
    type Error = String;

    #[inline]
    #[allow(clippy::cast_possible_truncation, clippy::as_conversions)]
    fn try_from(state: &WidgetState) -> Result<Self, Self::Error> {
        let width = state.get_as("width").ok_or("missing `width` attribute")?;
        let height = state.get_as("height").ok_or("missing `height` attribute")?;

        let idle_color = state
            .get_as("idle_color")
            .ok_or("missing `idle_color` attribute")?;

        let focus_color = state.get_as("focus_color").unwrap_or(idle_color);
        let active_color = state.get_as("active_color").unwrap_or(idle_color);

        let state = state.get_as("state").unwrap_or(ButtonState::Idle);

        Ok(Self {
            width,
            height,
            idle_color,
            focus_color,
            active_color,
            state,
        })
    }
}

impl widget::Runtime for ButtonRectangle {
    #[inline]
    fn attribute(&self, key: &str) -> Option<Value> {
        match key {
            "width" => Some(self.width.into()),
            "height" => Some(self.height.into()),
            "idle_color" => Some(self.idle_color.into()),
            "focus_color" => Some(self.focus_color.into()),
            "active_color" => Some(self.active_color.into()),
            _ => None,
        }
    }

    #[inline]
    #[allow(clippy::cast_possible_truncation, clippy::as_conversions)]
    fn attribute_mut(&mut self, key: &str, cb: fn(value: Option<&mut Value>)) {
        match key {
            "width" => match attribute_cb(self.width, cb).as_f64() {
                Some(width) => self.width = width as f32,
                None => todo!("logging"),
            },
            "height" => match attribute_cb(self.height, cb).as_f64() {
                Some(height) => self.height = height as f32,
                None => todo!("logging"),
            },
            _ => cb(None),
        }
    }

    #[inline]
    fn dimensions(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    #[inline]
    fn state(&self) -> WidgetState {
        let mut state = HashMap::new();
        state.insert("width", self.width.into());
        state.insert("height", self.height.into());
        state.insert("idle_color", self.idle_color.into());
        state.insert("focus_color", self.focus_color.into());
        state.insert("active_color", self.active_color.into());

        if let Ok(value) = serde_json::to_value(self.state) {
            state.insert("state", value);
        }

        WidgetState::new(widget::Kind::ButtonRectangle, state)
    }

    #[inline]
    fn interact(&mut self, event: &Event) -> Vec<event::Widget> {
        let mut output = vec![];

        match event {
            Event::Input(event::Input::Focus) => self.state = ButtonState::Focus,
            Event::Input(event::Input::Blur) => self.state = ButtonState::Idle,
            Event::Input(event::Input::MousePress { button, .. })
                if button == &event::MouseButton::Left =>
            {
                self.state = ButtonState::Active;
            }
            Event::Input(event::Input::MouseClick { button, .. })
                if button == &event::MouseButton::Left =>
            {
                self.state = ButtonState::Active;
                output.push(event::Widget::new("activated"));
            }
            _ if self.state == ButtonState::Active => self.state = ButtonState::Focus,
            _ => {}
        }

        output
    }

    #[inline]
    fn render(&self) -> Vec<Component> {
        let color = match self.state {
            ButtonState::Idle => self.idle_color,
            ButtonState::Focus => self.focus_color,
            ButtonState::Active => self.active_color,
        };

        let shape = Shape::Rectangle {
            width: self.width,
            height: self.height,
            color,
        };

        let component = Component {
            shape,
            coordinates: (0.0, 0.0),
        };

        vec![component]
    }
}

/// Run an attribute mutation callback provided by the callee.
fn attribute_cb(attribute: impl Into<Value>, cb: fn(value: Option<&mut Value>)) -> Value {
    let mut value = attribute.into();
    cb(Some(&mut value));

    value
}
