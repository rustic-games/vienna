#[derive(Debug)]
pub(super) struct Engine;

impl Default for Engine {
    fn default() -> Self {
        Self
    }
}

#[derive(Debug)]
pub(super) struct PluginStore {
    pub wasm: bool,
    pub mock: bool,
}

impl Default for PluginStore {
    fn default() -> Self {
        Self {
            wasm: true,
            mock: false,
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
    pub max_frames_per_second: Option<u64>,
}

impl Default for Renderer {
    fn default() -> Self {
        Self {
            max_frames_per_second: Some(60),
        }
    }
}
