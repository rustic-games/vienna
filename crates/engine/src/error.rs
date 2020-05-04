use crate::plugin::Func;
use displaydoc::Display;
use std::io;
use thiserror::Error;

/// Top-level error object exposing all possible error variants this crate can
/// produce.
#[derive(Debug, Display, Error)]
pub enum Error {
    /// plugin manager error
    PluginManager(#[from] PluginManagerError),

    /// plugin error
    Plugin(#[from] PluginError),

    /// unknown engine error
    Unknown,
}

/// Plugin related errors.
#[derive(Debug, Display, Error)]
pub enum PluginError {
    /// missing exported `{0}` function
    MissingExportedFunction(Func),

    /// invalid exported `{func}` function
    InvalidExportedFunction { func: Func, source: anyhow::Error },

    /// error running `{func}`
    RuntimeError { func: Func, source: wasmtime::Trap },
}

/// PluginManager related errors.
#[derive(Debug, Display, Error)]
pub enum PluginManagerError {
    /// inaccessible wasm module `{path}` ({kind:?})
    Io { path: String, kind: io::ErrorKind },

    /// invalid wasm module `{path}`
    InvalidModule { path: String, source: anyhow::Error },

    /// unknown wasm error for module `{path}`
    Unknown { path: String, source: anyhow::Error },

    /// error running plugin
    RuntimeException(#[from] PluginError),
}

impl From<(&str, anyhow::Error)> for PluginManagerError {
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
