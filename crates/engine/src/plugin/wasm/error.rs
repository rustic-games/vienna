use crate::plugin::Func;
use anyhow::Error;
use common::serde_json;
use std::io;
use std::path::PathBuf;
use thiserror::Error;
use wasmtime::Trap;

/// `WasmRuntime` related errors.
#[derive(Debug, Error)]
pub enum Runtime {
    #[error("missing exported `{0}` function")]
    MissingExportedFunction(Func),

    #[error("invalid exported `{func}` function")]
    InvalidExportedFunction { func: Func, source: Error },

    #[error("failed registration")]
    Registration,

    #[error("missing plugin name")]
    MissingName,

    #[error("cannot access runtime memory")]
    MemoryAccess,

    #[error("UTF-8 error")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("codec error")]
    Codec(#[from] serde_json::Error),

    #[error("plugin error")]
    Plugin(String),

    #[error("error running `{func}`")]
    Failed { func: Func, source: Trap },

    #[error("unable to run module")]
    InvalidModule(#[source] anyhow::Error),

    #[error("unknown wasm error")]
    Unknown(#[source] anyhow::Error),
}

impl From<std::num::TryFromIntError> for Runtime {
    fn from(_: std::num::TryFromIntError) -> Self {
        Self::MemoryAccess
    }
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
#[derive(Debug, Error)]
pub enum Handler {
    #[error("inaccessible wasm module `{path}` ({kind:?})")]
    Io { path: PathBuf, kind: io::ErrorKind },

    #[error("invalid wasm module `{path}`")]
    InvalidPlugin { path: PathBuf, source: Runtime },
}

impl From<(PathBuf, Runtime)> for Handler {
    fn from((path, source): (PathBuf, Runtime)) -> Self {
        Self::InvalidPlugin { path, source }
    }
}

impl From<(PathBuf, io::Error)> for Handler {
    fn from((path, err): (PathBuf, io::Error)) -> Self {
        Self::Io {
            path,
            kind: err.kind(),
        }
    }
}
