use crate::State;
use serde::{Deserialize, Serialize};

/// All details of the result of a `run` of the plugin.
#[derive(Debug, Default, Serialize, Deserialize)]
pub struct RunResult {
    /// Details about the error occurred while running the plugin.
    ///
    /// This returns `None` if no error occurred.
    pub error: Option<String>,

    /// The game state after the plugin finished running.
    pub state: State,
}
