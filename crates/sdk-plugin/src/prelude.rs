//! A set of useful default items exposed to plugins.

pub use crate::{widget, Sdk, State};
pub use anyhow::{self, bail, format_err, Result};
pub use common::{
    event, serde_json, Border, Canvas, Color, Deserialize, Event, Key, PluginState, Registration,
    Serialize, StateTransfer, Value,
};
