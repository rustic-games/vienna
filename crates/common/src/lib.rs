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
pub use shape::Shape;
pub use state::{
    Game as GameState, Plugin as PluginState, Transfer as StateTransfer, Widget as WidgetState,
    WidgetWithPosition,
};

// A list of third-party exposed types used by both the engine and SDK.
pub use serde::{de::DeserializeOwned, Deserialize, Serialize};
pub use serde_json::{self, Value};
