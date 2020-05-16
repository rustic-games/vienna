use crate::Value;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// The `Registration` type is used by plugins in the `init` function to expose
/// relevant details to the engine before the plugin is added to the engine's
/// runtime.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Registration {
    pub name: String,
    pub write: HashMap<String, Value>,
    pub read: HashMap<String, Vec<String>>,
}

impl Registration {
    pub fn new(name: impl Into<String>) -> Self {
        Registration {
            name: name.into(),
            write: HashMap::default(),
            read: HashMap::default(),
        }
    }

    pub fn write(mut self, key: impl Into<String>, value: Value) -> Self {
        self.write.insert(key.into(), value);
        self
    }

    pub fn read(mut self, plugin: impl Into<String>, keys: &[&'static str]) -> Self {
        let keys = keys.to_vec().into_iter().map(Into::into).collect();

        self.read.insert(plugin.into(), keys);
        self
    }
}
