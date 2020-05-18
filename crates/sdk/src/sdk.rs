use common::{Event, PluginState, StateTransfer, Value};
use std::collections::HashMap;

pub struct Sdk {
    /// The state of this plugin, which the plugin is allowed to mutate.
    pub(crate) owned_state: PluginState,

    /// The state of third-party plugins, which the plugin requested access to.
    ///
    /// This data can only be read, not mutated.
    borrowed_state: HashMap<String, PluginState>,

    /// A list of events that got triggered since the last update.
    events: Vec<Event>,

    /// A flag indicating if the `owned_state` has been modified.
    pub state_updated: bool,
}

impl<'a> Sdk {
    #[must_use]
    #[allow(clippy::missing_const_for_fn)] // false positive
    pub fn new(state: StateTransfer) -> Self {
        let StateTransfer {
            owned: owned_state,
            borrowed: borrowed_state,
            events,
        } = state;

        Self {
            owned_state,
            borrowed_state,
            events,
            state_updated: false,
        }
    }

    /// Get an immutable reference to a value owned by this plugin.
    pub fn get(&self, key: impl Into<String>) -> Option<&Value> {
        self.owned_state.get(&key.into())
    }

    /// Get a mutable reference to a value owned by this plugin.
    pub fn get_mut(&mut self, key: impl Into<String>) -> Option<&mut Value> {
        self.state_updated = true;

        self.owned_state.get_mut(&key.into())
    }

    /// Get a immutable reference to the value of another plugin.
    pub fn plugin(&self, name: impl Into<String>) -> Option<&PluginState> {
        self.borrowed_state.get(&name.into())
    }

    /// Get a list of events since the last update.
    #[must_use]
    pub fn events(&self) -> &[Event] {
        &self.events
    }
}
