use crate::{Deserialize, Event, Serialize, Value};
use std::collections::HashMap;

/// The state of the game.
///
/// Since the engine itself is agnostic to what state should be tracked, the
/// state itself lives in the [`PluginState`] types, which is created and
/// manipulated by plugins.
///
/// This struct stores that state, and hands off a mutable (for the plugin that
/// owns its `PluginState`) or an immutable (for plugins that want to read the
/// state of other plugins) reference to the relevant state objects.
#[derive(Debug, Default)]
pub struct GameState {
    state: HashMap<String, PluginState>,
}

impl GameState {
    /// Register the state of a plugin.
    pub fn register_plugin_state(&mut self, plugin: impl Into<String>, state: PluginState) {
        self.state.insert(plugin.into(), state);
    }

    /// Get an immutable reference to the state of a plugin.
    pub fn get(&self, plugin: impl Into<String>) -> Option<&PluginState> {
        self.state.get(&plugin.into())
    }

    /// Get a mutable reference to the state of a plugin.
    pub fn get_mut(&mut self, plugin: impl Into<String>) -> Option<&mut PluginState> {
        self.state.get_mut(&plugin.into())
    }
}

/// The state of a plugin.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
#[serde(transparent)]
pub struct PluginState {
    #[serde(rename = "s")]
    state: HashMap<String, Value>,
}

impl PluginState {
    /// Get an immutable reference to a value.
    pub fn get(&self, key: impl Into<String>) -> Option<&Value> {
        self.state.get(&key.into())
    }

    /// Get a mutable reference to a value.
    pub fn get_mut(&mut self, key: impl Into<String>) -> Option<&mut Value> {
        self.state.get_mut(&key.into())
    }
}

impl From<HashMap<String, Value>> for PluginState {
    fn from(state: HashMap<String, Value>) -> Self {
        Self { state }
    }
}

/// A collection of "owned" and "borrowed" plugin states, which get transfered
/// from the engine to the plugin.
///
/// This object owns the plugin states it encapsulates so that they can be
/// serialized and deserialized when moving across FFI boundaries.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct StateTransfer {
    #[serde(rename = "o")]
    pub owned: PluginState,
    #[serde(rename = "b")]
    pub borrowed: HashMap<String, PluginState>,
    #[serde(rename = "e")]
    pub events: Vec<Event>,
}

impl StateTransfer {
    /// Build a new [`StateTransfer`] object from a pointer and length to a JSON
    /// encoded vector of bytes.
    ///
    /// # Safety
    ///
    /// This requires `ptr` to point to the correct pointer, and `len` to be the
    /// correct length of the Vec.
    pub unsafe fn from_raw(ptr: *mut u8, len: usize) -> Self {
        let vec = Vec::from_raw_parts(ptr, len, len);
        serde_json::from_slice(&vec).unwrap()
    }
}
