//! The main crate of the Vienna engine.

#![warn(
    clippy::all,
    clippy::cargo,
    clippy::nursery,
    clippy::pedantic,
    clippy::restriction,
    future_incompatible,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rustdoc,
    unused
)]
#![allow(
    clippy::float_arithmetic,
    clippy::multiple_crate_versions,
    clippy::implicit_return,
    clippy::shadow_reuse
)]

mod builder;
mod config;
mod engine;
mod error;
mod plugin;
mod widget;

/// The core-coffee implementation.
#[cfg(all(feature = "core-coffee", not(feature = "core-ggez")))]
mod core {
    mod coffee;
    pub use self::coffee::*;
}

/// The core-ggez implementation.
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
