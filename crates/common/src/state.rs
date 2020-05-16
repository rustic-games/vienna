use crate::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub trait ReadState {
    /// Get a reference to a value in the store.
    ///
    /// This returns `None` if the key does not exist in the store.
    fn get(&self, key: &str) -> Option<&Value>;

    /// Get a reference to a float value in the store.
    ///
    /// This returns `None` if the key does not exist in the store, or the
    /// stored value for that key is of a different type.
    fn get_float(&self, key: &str) -> Option<&f64> {
        if let Some(Value::Float(v)) = self.get(key) {
            return Some(v);
        }
        None
    }

    /// Get a reference to a string value in the store.
    ///
    /// This returns `None` if the key does not exist in the store, or the
    /// stored value for that key is of a different type.
    fn get_string(&self, key: &str) -> Option<&String> {
        if let Some(Value::Str(v)) = self.get(key) {
            return Some(v);
        }
        None
    }
}

pub trait WriteState {
    /// Get a mutable reference to a value in the store.
    ///
    /// This returns `None` if the key does not exist in the store.
    fn get_mut(&mut self, key: &str) -> Option<&mut Value>;

    /// Get a mutable reference to a float value in the store.
    ///
    /// This returns `None` if the key does not exist in the store, or the
    /// stored value for that key is of a different type.
    fn get_float_mut(&mut self, key: &str) -> Option<&mut f64> {
        if let Some(Value::Float(v)) = self.get_mut(key) {
            return Some(v);
        }
        None
    }

    /// Get a mutable reference to a string value in the store.
    ///
    /// This returns `None` if the key does not exist in the store, or the
    /// stored value for that key is of a different type.
    fn get_string_mut(&mut self, key: &str) -> Option<&mut String> {
        if let Some(Value::Str(v)) = self.get_mut(key) {
            return Some(v);
        }
        None
    }
}

/// A set of data that is "owned" by some entity, and thus grants both read and
/// write access.
///
/// Notably, only existing state can be read or mutated, no new state can be
/// created.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct OwnedState {
    data: HashMap<String, Value>,
}

impl OwnedState {
    pub fn new(data: HashMap<String, Value>) -> Self {
        Self { data }
    }
}

impl ReadState for OwnedState {
    fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(&key.to_owned())
    }
}

impl WriteState for OwnedState {
    fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.data.get_mut(&key.to_owned())
    }
}

/// A set of data "borrowed" by an entity, with read-only access.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BorrowedState {
    data: HashMap<String, Value>,
}

impl BorrowedState {
    pub fn new(data: HashMap<String, Value>) -> Self {
        Self { data }
    }
}

impl ReadState for BorrowedState {
    fn get(&self, key: &str) -> Option<&Value> {
        self.data.get(&key.to_owned())
    }
}

impl From<OwnedState> for BorrowedState {
    fn from(state: OwnedState) -> Self {
        BorrowedState { data: state.data }
    }
}

/// The state of the game.
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct GameState {
    data: HashMap<String, OwnedState>,
}

impl GameState {
    /// Register a new key/value combination to store as game state.
    pub fn register_plugin_state(
        &mut self,
        plugin: impl Into<String>,
        data: HashMap<String, Value>,
    ) {
        self.data.insert(plugin.into(), OwnedState { data });
    }

    pub fn replace_plugin_state(&mut self, plugin: impl Into<String>, data: OwnedState) {
        self.data.insert(plugin.into(), data);
    }

    /// Get back an "owned" data container to which data can be written and
    /// read.
    pub fn owned_state(&mut self, plugin: impl Into<String>) -> Option<&mut OwnedState> {
        self.data.get_mut(&plugin.into())
    }

    pub fn borrowed_state(
        &self,
        plugin: impl Into<String>,
        keys: &[String],
    ) -> Option<BorrowedState> {
        self.data.get(&plugin.into()).map(|state| {
            let mut data = HashMap::default();
            for key in keys {
                if let Some(value) = state.get(key) {
                    data.insert(key.to_owned(), value.clone());
                }
            }

            BorrowedState { data }
        })
    }

    pub fn get(&self, plugin: impl Into<String>, key: impl Into<String>) -> Option<&Value> {
        self.data
            .get(&plugin.into())
            .and_then(|plugin| plugin.get(&key.into()))
    }
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct State {
    owned: OwnedState,
    borrowed: HashMap<String, BorrowedState>,
}

impl State {
    pub fn new(owned: OwnedState, borrowed: HashMap<String, BorrowedState>) -> Self {
        Self { owned, borrowed }
    }

    pub fn plugin(&self, plugin: impl Into<String>) -> Option<&BorrowedState> {
        self.borrowed.get(&plugin.into())
    }

    pub fn into_parts(self) -> (OwnedState, HashMap<String, BorrowedState>) {
        (self.owned, self.borrowed)
    }

    /// Build a new state object from a pointer and length to a JSON encoded
    /// vector of bytes.
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

impl ReadState for State {
    fn get(&self, key: &str) -> Option<&Value> {
        self.owned.get(key)
    }
}

impl WriteState for State {
    fn get_mut(&mut self, key: &str) -> Option<&mut Value> {
        self.owned.get_mut(key)
    }
}
