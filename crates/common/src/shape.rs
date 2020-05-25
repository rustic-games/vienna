//! Everything related to "primitive shapes" used to draw to the screen.

use crate::{Color, Deserialize, Serialize};

/// A list of primitive shapes the engine knows how to draw.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Shape {
    /// A circle with a given radius and color.
    Circle {
        /// Radius of the circle.
        radius: f32,

        /// The fill color of the circle.
        fill: Color,

        /// The border details.
        border: Option<Border>,
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

/// A border belonging to a shape.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Border {
    /// The color of the border.
    pub color: Color,

    /// The width of the border.
    pub width: f32,
}
