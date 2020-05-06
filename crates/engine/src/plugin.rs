mod wasm;

use crate::error::PluginError as Error;
use displaydoc::Display;
pub use wasm::Wasm;
use wasmtime::Instance;

type Result<T> = std::result::Result<T, Error>;

/// A list of exported functions the engine expects a plugin to have.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Display)]
pub enum Func {
    /// _run
    Run,
}

/// This trait makes it possible for a plugin to run to completion.
pub trait Plugin {
    type Error: std::error::Error;

    // TODO: untie this from the trait
    fn new(instance: Instance) -> Self;
    fn run(&self) -> Result<()>;
}
