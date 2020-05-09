use thiserror::Error;

/// Top-level error object exposing all possible error variants this crate can
/// produce.
#[derive(Debug, Error)]
pub enum Error {
    /// run error
    #[error(transparent)]
    Run(#[from] anyhow::Error),
}
