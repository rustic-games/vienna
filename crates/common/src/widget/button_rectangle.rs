use crate::{event, widget, Component, Deserialize, Event, Serialize, Value, WidgetState};
use std::{collections::HashMap, convert::TryFrom};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct ButtonRectangle {
    width: f32,
    height: f32,
}

impl TryFrom<&WidgetState> for ButtonRectangle {
    type Error = String;

    fn try_from(state: &WidgetState) -> Result<Self, Self::Error> {
        #[allow(clippy::cast_possible_truncation)]
        let width = state
            .get("width")
            .ok_or("missing `width` attribute")?
            .as_f64()
            .ok_or("`width` must be a number")? as f32;

        #[allow(clippy::cast_possible_truncation)]
        let height = state
            .get("height")
            .ok_or("missing `height` attribute")?
            .as_f64()
            .ok_or("`height` must be a number")? as f32;

        Ok(Self { width, height })
    }
}

impl widget::Runtime for ButtonRectangle {
    fn attribute(&self, key: &str) -> Option<Value> {
        match key {
            "width" => Some(self.width.into()),
            "height" => Some(self.height.into()),
            _ => None,
        }
    }

    fn attribute_mut(&mut self, key: &str, cb: fn(value: Option<&mut Value>)) {
        match key {
            "width" => {
                let mut value = Value::from(self.width);
                cb(Some(&mut value));

                #[allow(clippy::cast_possible_truncation)]
                let width = value.as_f64().expect("number") as f32;
                self.width = width;
            }
            "height" => {
                let mut value = Value::from(self.height);
                cb(Some(&mut value));

                #[allow(clippy::cast_possible_truncation)]
                let height = value.as_f64().expect("number") as f32;
                self.height = height;
            }
            _ => cb(None),
        }
    }

    fn dimensions(&self) -> (f32, f32) {
        (self.width, self.height)
    }

    fn state(&self) -> WidgetState {
        let mut state = HashMap::new();
        state.insert("width", self.width.into());
        state.insert("height", self.height.into());

        WidgetState::new(widget::Kind::ButtonRectangle, state)
    }

    // TODO
    fn interact(&mut self, _: &Event) -> Vec<event::Widget> {
        vec![]
    }

    // TODO
    fn render(&self) -> Vec<Component> {
        vec![]
    }
}
