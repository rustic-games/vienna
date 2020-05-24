//! Registration details of a plugin.

use crate::{widget, Deserialize, Serialize, Value, WidgetWithPosition};
use std::collections::HashMap;

/// The `Registration` type is used by plugins in the `init` function to expose
/// relevant details to the engine before the plugin is added to the engine's
/// runtime.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Registration {
    /// The name of the plugin.
    #[serde(rename = "n")]
    pub name: String,

    /// The state the plugin wants the engine to store in-between runs.
    #[serde(rename = "s")]
    pub state: Option<HashMap<String, Value>>,

    /// The widgets the plugin owns.
    #[serde(rename = "w")]
    pub widgets: Option<HashMap<String, WidgetWithPosition>>,

    /// A list of plugins this plugin depends on.
    ///
    /// A plugin can read the state of other plugins it depends on.
    #[serde(rename = "d")]
    pub dependencies: Option<Vec<String>>,
}

impl Registration {
    /// Create a new registration object.
    #[inline]
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            ..Self::default()
        }
    }

    /// Define a key/value pair of state this plugin wants to manage.
    #[inline]
    pub fn state(mut self, key: impl Into<String>, value: Value) -> Self {
        self.state
            .get_or_insert(HashMap::default())
            .insert(key.into(), value);

        self
    }

    /// Define a key/value pair of a widget this plugin wants to control.
    #[inline]
    #[must_use]
    pub fn widget(mut self, widget: widget::Builder) -> Self {
        let (name, widget) = widget.build();

        self.widgets
            .get_or_insert(HashMap::default())
            .insert(name, widget);

        self
    }

    /// Add a dependency to the plugin.
    #[inline]
    pub fn dependency(mut self, name: impl Into<String>) -> Self {
        self.dependencies.get_or_insert(vec![]).push(name.into());
        self
    }
}
