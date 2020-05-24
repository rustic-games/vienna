//! A set of widget-related exports and helper functions.

use common::widget::Builder;
pub use common::{
    widget::{Kind, Kind::*},
    WidgetWithPosition,
};

/// Create a new widget builder.
#[inline]
pub fn new(name: impl Into<String>, kind: Kind) -> Builder {
    Builder::new(name, kind)
}
