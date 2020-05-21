use thiserror::Error;

#[derive(Debug, Error)]
#[error("`WasmRuntime` related errors.")]
pub struct Runtime;

#[derive(Debug, Error)]
#[error("`WasmPlugin` related errors.")]
pub struct Handler;
