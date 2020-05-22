use common::{Canvas, Event, PluginState, StateTransfer, Value, Widget};
use std::collections::HashMap;

pub struct Data {
    pub sdk: Sdk,
    pub state: State,
    pub events: Vec<Event>,
}

impl From<StateTransfer> for Data {
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
    pub fn get(&self, key: impl Into<String>) -> Option<&Value> {
        self.owned.get(&key.into())
    }

    /// Get a mutable reference to a value owned by this plugin.
    pub fn get_mut(&mut self, key: impl Into<String>) -> Option<&mut Value> {
        self.updated = true;

        self.owned.get_mut(&key.into())
    }

    /// Get a mutable reference to a widget owned by this plugin.
    pub fn get_widget_mut(&mut self, key: impl Into<String>) -> Option<&mut Widget> {
        self.updated = true;

        self.owned.get_widget_mut(&key.into())
    }

    /// Get an immutable reference to the state of another plugin.
    pub fn plugin(&self, name: impl Into<String>) -> Option<&PluginState> {
        self.borrowed.get(&name.into())
    }
}

pub struct Sdk {
    canvas: Canvas,
}

impl Sdk {
    /// Get details about the window canvas.
    #[must_use]
    pub const fn canvas(&self) -> Canvas {
        self.canvas
    }
}
