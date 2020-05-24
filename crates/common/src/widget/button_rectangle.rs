//! A rectangular button.

use crate::{event, widget, Component, Deserialize, Event, Serialize, Value, WidgetState};
use std::{collections::HashMap, convert::TryFrom};

/// A (work in progress) rectangular button.
///
/// Once completed, this widget can be used to create interactive buttons for a
/// plugin.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ButtonRectangle {
    /// The width of the button.
    width: f32,

    /// The height of the button.
    height: f32,
}

impl TryFrom<&WidgetState> for ButtonRectangle {
    type Error = String;

    #[inline]
    #[allow(clippy::cast_possible_truncation, clippy::as_conversions)]
    fn try_from(state: &WidgetState) -> Result<Self, Self::Error> {
        let width = state
            .get("width")
            .ok_or("missing `width` attribute")?
            .as_f64()
            .ok_or("`width` must be a number")? as f32;

        let height = state
            .get("height")
            .ok_or("missing `height` attribute")?
            .as_f64()
            .ok_or("`height` must be a number")? as f32;

        Ok(Self { width, height })
    }
}

impl widget::Runtime for ButtonRectangle {
    #[inline]
    fn attribute(&self, key: &str) -> Option<Value> {
        match key {
            "width" => Some(self.width.into()),
            "height" => Some(self.height.into()),
            _ => None,
        }
    }

    #[inline]
    #[allow(clippy::cast_possible_truncation, clippy::as_conversions)]
    fn attribute_mut(&mut self, key: &str, cb: fn(value: Option<&mut Value>)) {
        match key {
            "width" => {
                let mut value = Value::from(self.width);
                cb(Some(&mut value));

                match value.as_f64() {
                    Some(width) => self.width = width as f32,
                    None => todo!("logging"),
                }
            }
            "height" => {
                let mut value = Value::from(self.height);
                cb(Some(&mut value));

                match value.as_f64() {
                    Some(height) => self.height = height as f32,
                    None => todo!("logging"),
                }
            }
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

        WidgetState::new(widget::Kind::ButtonRectangle, state)
    }

    // TODO
    #[inline]
    fn interact(&mut self, _: &Event) -> Vec<event::Widget> {
        vec![]
    }

    // TODO
    #[inline]
    fn render(&self) -> Vec<Component> {
        vec![]
    }
}
