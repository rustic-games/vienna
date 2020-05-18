use crate::{Deserialize, Serialize, Value};
use std::collections::HashMap;

/// The `Registration` type is used by plugins in the `init` function to expose
/// relevant details to the engine before the plugin is added to the engine's
/// runtime.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Registration {
    #[serde(rename = "n")]
    pub name: String,
    #[serde(rename = "s")]
    pub state: Option<HashMap<String, Value>>,
    #[serde(rename = "d")]
    pub dependencies: Option<Vec<String>>,
}

impl Registration {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }

    /// Define a key/value pair of state this plugin wants to manage.
    pub fn state(mut self, key: impl Into<String>, value: Value) -> Self {
        self.state
            .get_or_insert(HashMap::default())
            .insert(key.into(), value);

        self
    }

    pub fn dependency(mut self, name: impl Into<String>) -> Self {
        self.dependencies.get_or_insert(vec![]).push(name.into());
        self
    }
}
