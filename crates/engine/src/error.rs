use crate::plugin::wasm;
#[cfg(feature = "core-ggez")]
use ggez::error::GameError;
use std::io;
use thiserror::Error;

/// Top-level error object exposing all possible error variants this crate can
/// produce.
#[derive(Debug, Error)]
pub enum Error {
    #[error("engine builder error")]
    EngineBuilder(#[from] Builder),

    #[error("plugin handler error")]
    PluginHandler(#[from] Handler),

    #[cfg(feature = "core-ggez")]
    #[error("game error")]
    Game(#[from] GameError),
}

/// `EngineBuilder` related errors.
#[derive(Debug, Error)]
pub enum Builder {
    #[error("inaccessible plugin `{path}` ({kind:?})")]
    Io { path: String, kind: io::ErrorKind },

    #[error("plugin handler error")]
    PluginHandler(#[from] Handler),

    #[error("unknown builder error")]
    Unknown,
}

/// `plugin::Runtime` related errors.
#[derive(Debug, Error)]
pub enum Runtime {
    #[error("wasm runtime error")]
    WasmRuntime(#[from] wasm::RuntimeError),
}

/// `plugin::Handler` related errors.
#[derive(Debug, Error)]
pub enum Handler {
    #[error("wasm handler error")]
    WasmHandler(#[from] wasm::HandlerError),

    #[error(transparent)]
    Runtime(#[from] Runtime),
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

/// Game update related error
#[derive(Debug, Error)]
pub enum Updater {
    #[error("plugin runtime error")]
    PluginRuntime(#[from] Runtime),

    #[cfg(feature = "core-ggez")]
    #[error("game engine error")]
    GameEngine(#[from] GameError),
}
