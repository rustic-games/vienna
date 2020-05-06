use displaydoc::Display;
use thiserror::Error;

/// `WasmRuntime` related errors.
#[derive(Debug, Display, Error)]
pub struct Runtime;

/// `WasmPlugin` related errors.
#[derive(Debug, Display, Error)]
pub struct Handler;
