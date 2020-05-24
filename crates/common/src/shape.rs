//! Everything related to "primitive shapes" used to draw to the screen.

use crate::{Color, Deserialize, Serialize};

/// A list of primitive shapes the engine knows how to draw.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Shape {
    /// A circle with a given radius and color.
    Circle {
        /// Radius of the circle.
        radius: f32,

        /// Color of the circle.
        color: Color,
    },

    /// A rectangle with a width, height and color.
    Rectangle {
        /// The width of the rectangle.
        width: f32,

        /// The height of the rectangle.
        height: f32,

        /// The color of the rectangle.
        color: Color,
    },
}
