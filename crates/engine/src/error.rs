use displaydoc::Display;
use thiserror::Error;

#[derive(Debug, Display, Error)]
pub enum EngineError {
    /// unknown engine error
    Unknown,
}
