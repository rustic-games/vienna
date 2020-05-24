use crate::{Color, Deserialize, Serialize};

/// A list of primitive shapes the engine knows how to draw.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Shape {
    Line {
        length: f32,
        width: f32,
        color: Color,
    },

    Circle {
        radius: f32,
        color: Color,
    },

    Rectangle {
        width: f32,
        height: f32,
        color: Color,
    },
}
