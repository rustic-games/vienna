//! Mock errors.

use thiserror::Error;

/// A mock runtime error.
#[derive(Debug, Error)]
#[error("`WasmRuntime` related errors.")]
pub struct Runtime;

/// A mock handler error.
#[derive(Debug, Error)]
#[error("`WasmPlugin` related errors.")]
pub struct Handler;
