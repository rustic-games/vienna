#![warn(clippy::all, clippy::pedantic, clippy::nursery, clippy::cargo)]
#![allow(clippy::multiple_crate_versions)]

mod builder;
mod config;
mod engine;
mod error;
mod plugin;

#[cfg(all(feature = "core-coffee", not(feature = "core-ggez")))]
mod core {
    mod coffee;
    pub use self::coffee::*;
}

#[cfg(all(feature = "core-ggez", not(feature = "core-coffee")))]
mod core {
    mod ggez;
    pub use self::ggez::*;
}

use builder::Builder;

pub use error::Error;

/// A convenient top-level engine type exposed to start an engine with sensible
/// defaults.
pub type Engine = engine::Engine;
