use serde::{Deserialize, Serialize};

/// The `Registration` type is used by plugins in the `init` function to expose
/// relevant details to the engine before the plugin is added to the engine's
/// runtime.
#[derive(Debug, Serialize, Deserialize)]
pub struct Registration {
    pub name: String,
}

impl Registration {
    pub fn new(name: impl Into<String>) -> Self {
        Registration { name: name.into() }
    }
}
