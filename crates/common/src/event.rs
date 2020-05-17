use crate::{Deserialize, Serialize};
use std::collections::HashSet;

/// A list of keyboard keys supported by the engine.
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub enum Key {
    W,
    A,
    S,
    D,
}

/// A list of events the engine can trigger.
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum Event {
    Keyboard(HashSet<Key>),
    Mouse,
}