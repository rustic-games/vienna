pub(crate) mod wasm;

use displaydoc::Display;
pub use wasm::{Wasm, WasmManager};

/// A list of exported functions the engine expects a plugin to have.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Display)]
pub enum Func {
    /// _run
    Run,
}

/// A runtime is configured to run all methods required for a plugin to be
/// usable by the engine.
pub trait Runtime {
    type Data;
    type Error: std::error::Error + Send + Sync + 'static;

    fn new(data: Self::Data) -> Self;
    fn run(&self) -> Result<(), Self::Error>;
}

/// A handler takes ownership of external plugins, and runs them when requested.
pub trait Handler {
    type Error: std::error::Error + Send + Sync + 'static;

    /// Run all registered plugins.
    fn run_plugins(&mut self) -> Result<(), Self::Error>;

    /// Register a new plugin to handle.
    ///
    /// TODO: have this take `Into<Self::Plugin>` which would allow us to
    /// implement `From<Path>` for example for Wasm.
    fn register_plugin(&mut self, path: &str) -> Result<(), Self::Error>;
}
