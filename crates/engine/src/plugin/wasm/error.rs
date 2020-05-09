use crate::plugin::Func;
use anyhow::Error;
use displaydoc::Display;
use std::io;
use std::path::PathBuf;
use thiserror::Error;
use wasmtime::Trap;

/// `WasmRuntime` related errors.
#[derive(Debug, Display, Error)]
pub enum Runtime {
    /// missing exported `{0}` function
    MissingExportedFunction(Func),

    /// invalid exported `{func}` function
    InvalidExportedFunction { func: Func, source: Error },

    /// failed registration
    Registration,

    /// cannot access runtime memory
    MemoryAccess,

    /// UTF-8 error
    Utf8(#[from] std::str::Utf8Error),

    /// codec error
    Codec(#[from] serde_json::Error),

    /// error running `{func}`
    Failed { func: Func, source: Trap },

    /// invalid wasm module
    InvalidModule(#[source] anyhow::Error),

    /// unknown wasm error
    Unknown(#[source] anyhow::Error),
}

impl From<anyhow::Error> for Runtime {
    fn from(source: anyhow::Error) -> Self {
        let cause = source.to_string();
        if cause.contains("cross-`Store` instantiation is not currently supported") {
            return Self::InvalidModule(source);
        }
        if cause.contains("wrong number of imports provided") {
            return Self::InvalidModule(source);
        }
        if cause.starts_with("Bad") {
            return Self::InvalidModule(source);
        }
        if cause.starts_with("expected") {
            return Self::InvalidModule(source);
        }

        Self::Unknown(source)
    }
}

/// `WasmPlugin` related errors.
#[derive(Debug, Display, Error)]
pub enum Handler {
    /// inaccessible wasm module `{path}` ({kind:?})
    Io { path: PathBuf, kind: io::ErrorKind },
}

impl From<(PathBuf, io::Error)> for Handler {
    fn from((path, err): (PathBuf, io::Error)) -> Self {
        Self::Io {
            path,
            kind: err.kind(),
        }
    }
}
