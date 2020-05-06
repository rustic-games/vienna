use crate::plugin::wasm;
use displaydoc::Display;
use std::io;
use thiserror::Error;

/// Top-level error object exposing all possible error variants this crate can
/// produce.
#[derive(Debug, Display, Error)]
pub enum Error {
    /// engine builder error
    EngineBuilder(#[from] Builder),

    /// plugin handler error
    PluginHandler(#[from] Handler),

    /// plugin runtime error
    PluginRuntime(#[from] Runtime),

    /// unknown engine error
    Unknown,
}

/// `EngineBuilder` related errors.
#[derive(Debug, Display, Error)]
pub enum Builder {
    /// inaccessible plugin `{path}` ({kind:?})
    Io { path: String, kind: io::ErrorKind },

    /// plugin handler error
    PluginHandler(#[from] Handler),

    /// unknown builder error
    Unknown,
}

/// `plugin::Runtime` related errors.
#[derive(Debug, Display, Error)]
pub enum Runtime {
    /// wasm runtime error
    WasmRuntime(#[from] wasm::RuntimeError),
}

/// `plugin::Handler` related errors.
#[derive(Debug, Display, Error)]
pub enum Handler {
    /// wasm handler error
    WasmHandler(#[from] wasm::HandlerError),
}

impl From<walkdir::Error> for Builder {
    fn from(err: walkdir::Error) -> Self {
        use std::borrow::Cow;
        use std::path::Path;

        let path = err
            .path()
            .map(Path::to_string_lossy)
            .map(Cow::into_owned)
            .unwrap_or_default();

        if let Some(err) = err.io_error() {
            let kind = err.kind();
            return Self::Io { path, kind };
        };

        Self::Unknown
    }
}
