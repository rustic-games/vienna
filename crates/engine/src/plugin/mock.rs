//! A mock implementation of the plugin system used for testing.

mod error;
mod manager;
mod plugin;

pub use error::{Handler as HandlerError, Runtime as RuntimeError};
pub use manager::Manager;
pub use plugin::Plugin;
