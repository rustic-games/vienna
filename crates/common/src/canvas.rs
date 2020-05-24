//! Details about the canvas within which the game is rendered.

use crate::{Deserialize, Serialize};

/// Canvas details.
#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct Canvas {
    /// The width of the canvas.
    width: u16,

    /// The height of the canvas.
    height: u16,
}

impl Canvas {
    /// Create a new canvas.
    #[inline]
    #[must_use]
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    /// Get the dimensions (width, height) of the canvas.
    #[inline]
    #[must_use]
    pub const fn dimensions(self) -> (u16, u16) {
        (self.width, self.height)
    }
}
