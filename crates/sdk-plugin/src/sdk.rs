//! Types used to convert and expose SDK functionality.

use common::{
    serde_json, Canvas, DeserializeOwned, Event, PluginState, StateTransfer, Value,
    WidgetWithPosition,
};
use std::collections::HashMap;

/// A data container used to unwrap data transfered from the engine to the
/// plugin.
pub struct Data {
    /// Provides access to top-level SDK helper methods.
    pub sdk: Sdk,

    /// The state of the plugin as reported by the engine.
    pub state: State,

    /// The events reported by the engine to have been emitted since this plugin
    /// last ran.
    // TODO:
    //
    // Move events into `sdk`, and have it expose helper methods to get events
    // of a named widget, or get a slice of all available events.
    pub events: Vec<Event>,
}

impl From<StateTransfer> for Data {
    #[inline]
    fn from(transfer: StateTransfer) -> Self {
        let StateTransfer {
            owned,
            borrowed,
            events,
            canvas,
        } = transfer;

        let sdk = Sdk { canvas };
        let state = State {
            owned,
            borrowed,
            updated: false,
        };

        Self { sdk, state, events }
    }
}

/// The state of the plugin.
pub struct State {
    /// The state of this plugin, which the plugin is allowed to mutate.
    pub(super) owned: PluginState,

    /// The state of third-party plugins, which the plugin requested access to.
    ///
    /// This data can only be read, not mutated.
    borrowed: HashMap<String, PluginState>,

    /// A flag indicating if the `owned_state` has been modified.
    pub updated: bool,
}

impl State {
    /// Get an immutable reference to a value owned by this plugin.
    #[inline]
    pub fn get(&self, key: impl Into<String>) -> Option<&Value> {
        self.owned.get(&key.into())
    }

    /// Get a mutable reference to a value owned by this plugin.
    #[inline]
    pub fn get_mut(&mut self, key: impl Into<String>) -> Option<&mut Value> {
        self.updated = true;

        self.owned.get_mut(&key.into())
    }

    /// Get an owned state value of a specific type.
    #[inline]
    pub fn get_as<T: DeserializeOwned>(&self, key: impl Into<String>) -> Option<T> {
        self.get(key)
            .cloned()
            .and_then(|v| serde_json::from_value(v).ok())
    }

    /// Get a mutable reference to a widget owned by this plugin.
    #[inline]
    pub fn get_widget_mut(&mut self, key: impl Into<String>) -> Option<&mut WidgetWithPosition> {
        self.updated = true;

        self.owned.get_widget_mut(&key.into())
    }

    /// Get an immutable reference to the state of another plugin.
    #[inline]
    pub fn plugin(&self, name: impl Into<String>) -> Option<&PluginState> {
        self.borrowed.get(&name.into())
    }
}

/// The top-level SDK helper struct.
pub struct Sdk {
    /// The game screen canvas.
    canvas: Canvas,
}

impl Sdk {
    /// Get details about the window canvas.
    #[inline]
    #[must_use]
    pub const fn canvas(&self) -> Canvas {
        self.canvas
    }
}
