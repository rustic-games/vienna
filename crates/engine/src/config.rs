//! Objects to encapsulate configurations of parts of the engine.

use common::Canvas;

/// Top-level engine configuration.
#[derive(Debug)]
pub(super) struct Engine {
    /// The canvas the engine draws on.
    pub canvas: Canvas,
}

impl From<Canvas> for Engine {
    fn from(canvas: Canvas) -> Self {
        Self { canvas }
    }
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            canvas: Canvas::new(800, 600),
        }
    }
}

/// Configuration for the "updater" system.
#[derive(Debug)]
pub(super) struct Updater {
    /// The amount of updates per second the engine will do.
    ///
    /// This also means each registered plugin will run as much as this value is
    /// set to.
    pub updates_per_second: u64,
}

impl Default for Updater {
    fn default() -> Self {
        Self {
            updates_per_second: 100,
        }
    }
}

/// Configuration for the "renderer" system.
#[derive(Debug)]
pub(super) struct Renderer {
    /// The amount of frames per second the renderer will run.
    pub max_frames_per_second: Option<u16>,
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            max_frames_per_second: Some(90),
        }
    }
}
