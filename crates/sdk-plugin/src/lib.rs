#![warn(
    clippy::all,
    clippy::pedantic,
    clippy::nursery,
    clippy::cargo,
    future_incompatible,
    nonstandard_style,
    rust_2018_compatibility,
    rust_2018_idioms,
    rustdoc,
    unused
)]
#![allow(clippy::multiple_crate_versions)]

mod error;
mod internal;
mod plugin;
pub mod prelude;
mod sdk;
pub mod widget;

pub use error::Error;
pub use internal::{init, malloc, run};
pub use sdk::{Data, Sdk, State};