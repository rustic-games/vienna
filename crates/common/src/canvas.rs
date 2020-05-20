use crate::{Deserialize, Serialize};

#[derive(Debug, Default, Copy, Clone, Serialize, Deserialize)]
pub struct Canvas {
    width: u16,
    height: u16,
}

impl Canvas {
    #[must_use]
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    #[must_use]
    pub const fn width(self) -> u16 {
        self.width
    }

    #[must_use]
    pub const fn height(self) -> u16 {
        self.height
    }

    #[must_use]
    pub const fn dimensions(self) -> (u16, u16) {
        (self.width, self.height)
    }
}
