use crate::error::Builder as Error;
use crate::plugin::{wasm, Handler};
use crate::Engine;
use common::GameState;
use std::path::PathBuf;

type Result<T> = std::result::Result<T, Error>;

/// A builder used to create an [`Engine`].
#[derive(Debug, Default)]
pub struct Builder<'a> {
    plugin_paths: Vec<&'a str>,
    game_state: Option<GameState>,
}

impl<'a> Builder<'a> {
    /// Add a path from which *.wasm plugins are loaded.
    ///
    /// How it works:
    ///
    /// - The entire directory tree of the path is searched for plugins.
    /// - A plugin is any file that has the "wasm" extension.
    /// - Duplicate file names are ignored (even for different paths).
    pub fn with_plugin_path(mut self, path: &'a str) -> Self {
        self.plugin_paths.push(path);
        self
    }

    /// Use an existing game state.
    ///
    /// This can be used to resume an active game session.
    pub fn with_game_state(mut self, game_state: GameState) -> Self {
        self.game_state = Some(game_state);
        self
    }

    /// Build the final [`Engine`].
    ///
    /// # Errors
    ///
    /// Returns an error if anything is misconfigured.
    #[cfg(all(feature = "core-ggez", not(feature = "core-coffee")))]
    pub fn build(self) -> Result<Engine> {
        self.build_inner()
    }

    // FIXME: this is a temporary solution until a better one is found.
    //
    // Since `coffee` must initialize the `Engine` by itself, we have to somehow
    // provide it with the configuration set in this builder.
    //
    // For now, this is done through a mutable static variable that is set once
    // in the builder, and then consumed when starting the engine.
    //
    // Ideally the builder:
    //
    // 1. Is engine agnostic and doesn't need any conditional compilation.
    // 2. Does not need unsafe code and a global mutable variable to function.
    #[cfg(all(feature = "core-coffee", not(feature = "core-ggez")))]
    pub fn build(self) -> Result<Engine> {
        use crate::core::{Config, CONFIG};

        let plugin_paths = self.plugin_paths.into_iter().map(str::to_owned).collect();
        let game_state = self.game_state;

        let config = Config::new(plugin_paths, game_state);

        unsafe { CONFIG.set(config).unwrap() };
        Ok(Engine::default())
    }

    pub(super) fn build_inner(self) -> Result<Engine> {
        let mut game_state = self.game_state.unwrap_or_default();
        let mut plugin_handler = Box::new(wasm::Manager::default());

        for path in &self.plugin_paths {
            for plugin in find_plugins_in_path(path)? {
                plugin_handler.register_plugin(&mut game_state, &plugin)?;
            }
        }

        Ok(Engine {
            plugin_handler,
            game_state,
            ..Engine::default()
        })
    }
}

/// Find all files ending in *.wasm within the given path.
///
/// Files with duplicate names are ignored. Even if two plugins reside in
/// different directories, if their names are equal, only the first one is added
/// to the list of plugins.
fn find_plugins_in_path(path: &str) -> Result<Vec<PathBuf>> {
    use std::collections::HashSet;
    use std::ffi::OsStr;
    use walkdir::WalkDir;

    let mut paths = vec![];
    let mut duplicates = HashSet::new();

    for entry in WalkDir::new(path) {
        let entry = entry?;

        if !entry.file_type().is_file() {
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
mod tests {
    use super::*;

    mod with_plugin_path {
        use super::*;

        #[test]
        fn works() {
            let builder = Builder::default();
            let builder = builder.with_plugin_path("foo");

            assert_eq!(builder.plugin_paths.get(0), Some(&"foo"));
        }
    }

    mod build {
        use super::*;
        use common::Value;
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
            let mut plugin_state = HashMap::default();
            plugin_state.insert("bar".to_owned(), Value::String("baz".to_owned()));
            game_state.register_plugin_state("foo", plugin_state.into());

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
