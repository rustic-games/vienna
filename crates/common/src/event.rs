//! Events used to communicate between player, plugin and widget.

use crate::{Deserialize, Serialize, Value};
use std::collections::{HashMap, HashSet};

/// A list of events the engine can trigger.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Event {
    /// Input events originate from the player.
    ///
    /// Widgets consume input events, and (optionally) transform them to widget
    /// events.
    Input(Input),

    /// Widget events originate from widgets.
    ///
    /// Plugins consume widget events and transform their state based on these
    /// events.
    Widget {
        /// The name of widget to which this event belongs.
        ///
        /// This is used by plugins to match events against specific widgets
        /// they own, so that events can be applied to the correct widget, if
        /// needed.
        name: String,

        /// Details about the widget event.
        event: Widget,
    },
}

/// An event triggered via an input method.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Input {
    /// A change in pointer position.
    Pointer(f32, f32),

    /// A keyboard key event.
    Keyboard {
        /// A set of keys captured in the input event.
        keys: HashSet<Key>,
    },

    /// A mouse button event.
    Mouse(Mouse),
}

/// An event triggered via the mouse.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Mouse {
    /// TODO
    button: (),
}

/// An event triggered by a widget.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Widget {
    /// The name of the event the widget triggered.
    name: String,

    /// Structured data attached to an event.
    ///
    /// Widgets can use attributes to enrich events with contextual data. For
    /// example, a "move" event can contain an "axis" and "amount" attribute to
    /// signal to the plugin in which direction and for how much a widget should
    /// be moved.
    attributes: HashMap<String, Value>,
}

impl Widget {
    /// Create a new widget event.
    #[inline]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            attributes: HashMap::default(),
        }
    }

    /// Get the name of the event.
    #[inline]
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get a structured attribute attached to a widget event.
    #[inline]
    pub fn attribute(&self, key: impl Into<String>) -> Option<&Value> {
        self.attributes.get(&key.into())
    }

    /// Add a new attribute to the event.
    #[inline]
    pub fn add_attribute<T: serde::ser::Serialize>(&mut self, key: impl Into<String>, value: T) {
        #[allow(clippy::match_wild_err_arm)]
        match serde_json::to_value(value) {
            Ok(value) => self.attributes.insert(key.into(), value),
            Err(_) => todo!("logging"),
        };
    }
}

/// A list of keyboard keys supported by the engine.
#[derive(Debug, Clone, Copy, Hash, Eq, PartialEq, Serialize, Deserialize)]
#[allow(clippy::missing_docs_in_private_items)]
pub enum Key {
    // letter keys
    A,
    B,
    D,
    E,
    G,
    Q,
    R,
    S,
    W,

    // other keys
    Minus,
    Plus,

    // modifier keys
    Ctrl,
    Shift,
}
