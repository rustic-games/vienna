mod button_rectangle;
mod moving_circle;

use crate::{
    event, Component, Deserialize, Event, Serialize, Value, WidgetState, WidgetWithPosition,
};
pub use button_rectangle::ButtonRectangle;
pub use moving_circle::MovingCircle;
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Kind {
    MovingCircle,
    ButtonRectangle,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Widget {
    MovingCircle(WidgetState),
    ButtonRectangle(WidgetState),
}

impl From<WidgetState> for Widget {
    fn from(state: WidgetState) -> Self {
        match state.kind() {
            Kind::MovingCircle => Self::MovingCircle(state),
            Kind::ButtonRectangle => Self::ButtonRectangle(state),
        }
    }
}

pub struct Builder {
    name: String,
    kind: Kind,
    visible: bool,
    position: (f32, f32),
    attributes: HashMap<String, Value>,
}

impl Builder {
    #[must_use]
    pub fn new(name: impl Into<String>, kind: Kind) -> Self {
        Self {
            name: name.into(),
            kind,
            visible: true,
            position: (0.0, 0.0),
            attributes: HashMap::default(),
        }
    }

    #[must_use]
    pub fn attribute(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    #[must_use]
    pub const fn hidden(mut self) -> Self {
        self.visible = false;
        self
    }

    #[must_use]
    pub const fn position(mut self, x: f32, y: f32) -> Self {
        self.position = (x, y);
        self
    }

    #[must_use]
    pub fn build(self) -> (String, WidgetWithPosition) {
        let widget = WidgetState::new(self.kind, self.attributes);

        (
            self.name,
            WidgetWithPosition::new(self.position, self.visible, widget),
        )
    }
}

pub trait Runtime {
    /// Get the value of an attribute of this widget.
    ///
    /// This returns an owned value, so the attribute might be cloned each time
    /// this method is called.
    ///
    /// Returns none if the attribute does not exist.
    fn attribute(&self, key: &str) -> Option<Value>;

    /// Allows mutating an attribute of the widget.
    ///
    /// This method takes a callback, which receives an option with a mutable
    /// reference to the value. If the option is `None`, this means the provided
    /// attribute key does not exist.
    fn attribute_mut(&mut self, key: &str, cb: fn(value: Option<&mut Value>));

    /// The boxed dimensions (width, height) of the widget, to detect mouse-over
    /// events.
    fn dimensions(&self) -> (f32, f32);

    /// The current state of the widget.
    ///
    /// This is used to serialize widgets in save games or when transferring
    /// state between the engine and plugins.
    fn state(&self) -> WidgetState;

    /// Render is called when the engine wants to draw the widget.
    ///
    /// The widget exposes a set of "components", which instruct the engine what
    /// it should look like.
    fn render(&self) -> Vec<Component>;

    /// Whenever a player interacts with a widget, the `interact` method is
    /// called. The event contains the interaction type (e.g. mouse-over, key
    /// press, etc.).
    ///
    /// When a widget acts on an interaction, it can itself trigger one or more
    /// events based on that interaction.
    ///
    /// For example, on a LMB-up event, a "button" widget emits the
    /// "triggered" widget event as output.
    ///
    /// By default a widget is non-interactive.
    #[allow(unused)]
    fn interact(&mut self, event: &Event) -> Vec<event::Widget> {
        vec![]
    }
}
