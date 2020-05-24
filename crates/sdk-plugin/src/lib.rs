//! The Vienna SDK used to build Rust-based `WebAssembly` plugins.

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

mod error;
mod internal;
mod plugin;
pub mod prelude;
mod sdk;
pub mod widget;

pub use error::Error;
pub use internal::{init, malloc, run};
pub use sdk::{Data, Sdk, State};
