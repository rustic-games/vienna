//! A set of errors used by the SDK.

use common::serde_json;
use thiserror::Error;

/// Top-level error object exposing all possible error variants this crate can
/// produce.
#[derive(Debug, Error)]
pub enum Error {
    /// codec error
    #[error("codec error")]
    Codec(#[from] serde_json::Error),

    /// run error
    #[error(transparent)]
    Run(#[from] anyhow::Error),
}
