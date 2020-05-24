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

/// The backend-coffee implementation.
#[cfg(all(feature = "backend-coffee", not(feature = "backend-ggez")))]
mod backend {
    mod coffee;
    pub use self::coffee::*;
}

/// The backend-ggez implementation.
#[cfg(all(feature = "backend-ggez", not(feature = "backend-coffee")))]
mod backend {
    mod ggez;
    pub use self::ggez::*;
}

use builder::Builder;

pub use error::Error;

/// A convenient top-level engine type exposed to start an engine with sensible
/// defaults.
pub type Engine = engine::Engine;
