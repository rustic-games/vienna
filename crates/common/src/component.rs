//! Component related items.

use crate::{Deserialize, Serialize, Shape};

/// A piece of a widget.
///
/// Each widget consists of one or more components.
///
/// A component consists of one primitive shape, and the position of that shape
/// relative to the top-left of the widget.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Component {
    /// The shape of the widget component.
    pub shape: Shape,

    /// The relative position of the component measuring from the top-left of
    /// the widget.
    pub coordinates: (f32, f32),
}
