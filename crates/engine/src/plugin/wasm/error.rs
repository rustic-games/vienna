use crate::plugin::Func;
use anyhow::Error;
use displaydoc::Display;
use std::io;
use thiserror::Error;
use wasmtime::Trap;

/// `WasmRuntime` related errors.
#[derive(Debug, Display, Error)]
pub enum Runtime {
    /// missing exported `{0}` function
    MissingExportedFunction(Func),

    /// invalid exported `{func}` function
    InvalidExportedFunction { func: Func, source: Error },

    /// error running `{func}`
    Failed { func: Func, source: Trap },
}

/// `WasmPlugin` related errors.
#[derive(Debug, Display, Error)]
pub enum Handler {
    /// inaccessible wasm module `{path}` ({kind:?})
    Io { path: String, kind: io::ErrorKind },

    /// invalid wasm module `{path}`
    InvalidModule { path: String, source: anyhow::Error },

    /// error running wasm instance
    Runtime(#[from] Runtime),

    /// unknown wasm error for module `{path}`
    Unknown { path: String, source: anyhow::Error },
}

impl From<(&str, anyhow::Error)> for Handler {
    fn from((path, source): (&str, anyhow::Error)) -> Self {
        let path = path.to_owned();

        for cause in source.chain() {
            if let Some(source) = cause.downcast_ref::<io::Error>() {
                let kind = source.kind();
                return Self::Io { path, kind };
            }
        }

        let cause = source.to_string();
        if cause.contains("cross-`Store` instantiation is not currently supported") {
            return Self::InvalidModule { path, source };
        }
        if cause.contains("wrong number of imports provided") {
            return Self::InvalidModule { path, source };
        }
        if cause.starts_with("Bad") {
            return Self::InvalidModule { path, source };
        }
        if cause.starts_with("expected") {
            return Self::InvalidModule { path, source };
        }

        Self::Unknown { path, source }
    }
}
