//! The result of the run of a plugin.

use crate::StateTransfer;
use serde::{Deserialize, Serialize};

/// All details of the result of a `run` of the plugin.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RunResult {
    /// Details about the error occurred while running the plugin.
    ///
    /// This returns `None` if no error occurred.
    #[serde(rename = "e")]
    pub error: Option<String>,

    /// The game state after the plugin finished running.
    #[serde(rename = "s")]
    pub state: Option<StateTransfer>,
}
