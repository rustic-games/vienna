//! A common set of types and functions used by Vienna.

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

mod canvas;
mod color;
mod component;
pub mod event;
mod registration;
mod run_result;
mod shape;
mod state;
pub mod widget;

pub use canvas::Canvas;
pub use color::Color;
pub use component::Component;
pub use event::{Event, Key};
pub use registration::Registration;
pub use run_result::RunResult;
pub use shape::{Border, Shape};
pub use state::{
    Game as GameState, Plugin as PluginState, Transfer as StateTransfer, Widget as WidgetState,
    WidgetWithPosition,
};

// A list of third-party exposed types used by both the engine and SDK.
pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub use serde_json::{self, Value};
