//! The main way to create a valid game engine instance.

use crate::{
    config,
    error::Builder as Error,
    plugin::{wasm, Handler},
    Engine,
};
use common::{Canvas, GameState};
use std::{mem, path::PathBuf};

/// Convenient way to create an [`Engine`].
///
/// # Example
///
/// ```
/// # use vienna_engine::Engine;
/// Engine::builder()
///     .with_plugin_path("plugins")
///     .with_window_dimensions(800, 600)
///     .with_vsync()
///     .build()?;
/// # Ok::<(), vienna_engine::error::Builder>(())
/// ```
#[derive(Debug, Default)]
pub struct Builder {
    /// A list of paths in which to search for wasm plugins.
    plugin_paths: Vec<PathBuf>,

    /// The state of a game (e.g. a saved game state)
    game_state: GameState,

    /// The maximum number of frames per second to run the game at.
    maximum_fps: Option<u16>,

    // These are exported so that the `coffee` core's `run` function has access
    // to the values when creating a new window.
    /// Details about the canvas of the game.
    pub(crate) canvas: Canvas,

    /// Whether or not to enable vsync.
    pub(crate) vsync_enabled: bool,
}

impl Builder {
    /// Add a path from which *.wasm plugins are loaded.
    ///
    /// How it works:
    ///
    /// - The entire directory tree of the path is searched for plugins.
    /// - A plugin is any file that has the "wasm" extension.
    /// - Duplicate file names are ignored (even for different paths).
    pub fn with_plugin_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.plugin_paths.push(path.into());
        self
    }

    /// Use an existing game state.
    ///
    /// This can be used to resume an active game session.
    #[allow(clippy::missing_const_for_fn)]
    pub fn with_game_state(mut self, game_state: GameState) -> Self {
        self.game_state = game_state;
        self
    }

    /// Configure the width and height of the window.
    pub const fn with_window_dimensions(mut self, width: u16, height: u16) -> Self {
        self.canvas = Canvas::new(width, height);
        self
    }

    /// Limit the frames per seconds to be equal or less than the refresh rate
    /// of the monitor.
    ///
    /// Use this to limit screen-tearing.
    pub const fn with_vsync(mut self) -> Self {
        self.vsync_enabled = true;
        self
    }

    /// The maximum frames the engine should render per second.
    ///
    /// Set this to reduce the system resources used by the engine.
    ///
    /// # Vertical synchronization
    ///
    /// If [`with_vsync()`] is provided, the frames per second will be
    /// capped at the refresh rate of the active monitor, unless the maximum
    /// frames per second is set at a lower value.
    ///
    /// Defaults to unlimited frames per second.
    pub fn with_maximum_fps(mut self, fps: u16) -> Self {
        self.maximum_fps = match fps {
            0 => None,
            fps => Some(fps),
        };

        self
    }

    /// Build the final [`Engine`].
    ///
    /// # Errors
    ///
    /// Returns an error if anything is misconfigured.
    #[cfg(all(feature = "core-ggez", not(feature = "core-coffee")))]
    pub fn build(mut self) -> Result<Engine, Error> {
        self.build_inner()
    }

    /// FIXME: this is a temporary solution until a better one is found.
    ///
    /// Since `coffee` must initialize the `Engine` by itself, we have to somehow
    /// provide it with the configuration set in this builder.
    ///
    /// See: <https://github.com/hecrj/coffee/issues/72>
    ///
    /// For now, this is done through a mutable static variable that is set once
    /// in the builder, and then consumed when starting the engine.
    ///
    /// Ideally the builder:
    ///
    /// 1. Is engine agnostic and doesn't need any conditional compilation.
    /// 2. Does not need unsafe code and a global mutable variable to function.
    ///
    /// The current path to starting the coffee engine is:
    ///
    /// - Use builder and trigger `build`
    ///   - Store builder in global variable
    ///   - Return engine with default configuration
    /// - Use engine's `start` method
    ///   - Run `coffee::run`
    ///      - Fetch window config from global builder
    ///   - Run coffee's `Game::load`
    ///   - Load global builder
    ///   - Construct new builder with global config
    ///   - Call `build_inner` to create engine
    ///   - Start engine
    ///
    #[cfg(all(feature = "core-coffee", not(feature = "core-ggez")))]
    pub fn build(self) -> Result<Engine, Error> {
        use crate::core::BUILDER;

        if unsafe { BUILDER.set(self) }.is_err() {
            todo!("logging")
        }

        Ok(Engine::default())
    }

    /// Actual logic to build the engine.
    ///
    /// This is split from the regular `build()` method because that method
    /// are implemented differently based on the enabled core.
    pub(super) fn build_inner(&mut self) -> Result<Engine, Error> {
        let mut game_state = mem::take(&mut self.game_state);
        let mut plugin_handler = Box::new(wasm::Manager::default());

        for path in &self.plugin_paths {
            for plugin in find_plugins_in_path(path)? {
                plugin_handler.register_plugin(&mut game_state, &plugin)?;
            }
        }

        let renderer = From::from(config::Renderer {
            max_frames_per_second: self.maximum_fps,
        });

        Ok(Engine {
            config: self.canvas.into(),
            plugin_handler,
            game_state,
            renderer,
            ..Engine::default()
        })
    }
}

/// Find all files ending in *.wasm within the given path.
///
/// Files with duplicate names are ignored. Even if two plugins reside in
/// different directories, if their names are equal, only the first one is added
/// to the list of plugins.
fn find_plugins_in_path(path: &PathBuf) -> Result<Vec<PathBuf>, Error> {
    use std::collections::HashSet;
    use std::ffi::OsStr;
    use walkdir::WalkDir;

    let mut paths = vec![];
    let mut duplicates = HashSet::new();

    for entry in WalkDir::new(path) {
        let entry = entry?;

        if entry.file_type().is_dir() {
            continue;
        }

        let path = entry.path();
        if path.extension().and_then(OsStr::to_str) != Some("wasm") {
            continue;
        }

        if let Some(file) = path.file_name().and_then(OsStr::to_str) {
            if duplicates.contains(file) {
                continue;
            }

            paths.push(path.to_owned());
            duplicates.insert(file.to_owned());
        }
    }

    Ok(paths)
}

#[cfg(test)]
#[allow(clippy::restriction)]
mod tests {
    use super::*;

    mod with_plugin_path {
        use super::*;

        #[test]
        fn works() {
            let builder = Builder::default();
            let builder = builder.with_plugin_path("foo");

            assert_eq!(builder.plugin_paths.get(0), Some(&"foo".into()));
        }
    }

    mod build {
        use super::*;
        use common::{PluginState, Value};
        use std::collections::HashMap;
        use tempfile::NamedTempFile;

        #[test]
        fn without_paths() {
            let builder = Builder::default();

            assert!(builder.build().is_ok())
        }

        #[test]
        fn with_valid_path() {
            let file = NamedTempFile::new().expect("temporary file");

            let builder = Builder::default();
            let builder = builder.with_plugin_path(file.path().to_str().unwrap());

            assert!(builder.build().is_ok())
        }

        #[test]
        fn with_invalid_path() {
            let builder = Builder::default();
            let builder = builder.with_plugin_path("foo");

            let err = builder.build().unwrap_err();

            assert_eq!(
                err.to_string(),
                "inaccessible plugin `foo` (NotFound)".to_owned(),
            )
        }

        #[test]
        fn with_game_state() {
            let mut game_state = GameState::default();
            let widgets: HashMap<&str, _> = HashMap::default();
            let mut state = HashMap::default();
            state.insert("bar", "baz");

            let plugin_state = PluginState::new(state, widgets);

            game_state.register_plugin_state("foo", plugin_state);

            let builder = Builder::default();
            let builder = builder.with_game_state(game_state);
            let engine = builder.build().unwrap();

            assert_eq!(
                engine
                    .game_state
                    .get("foo")
                    .and_then(|plugin| plugin.get("bar")),
                Some(&Value::String("baz".to_owned()))
            );
        }
    }
}
