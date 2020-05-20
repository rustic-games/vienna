use common::Canvas;

#[derive(Debug)]
pub(super) struct Engine {
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

#[derive(Debug)]
pub(super) struct Updater {
    pub updates_per_second: u64,
}

impl Default for Updater {
    fn default() -> Self {
        Self {
            updates_per_second: 100,
        }
    }
}

#[derive(Debug)]
pub(super) struct Renderer {
    pub max_frames_per_second: Option<u16>,
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            max_frames_per_second: Some(90),
        }
    }
}
