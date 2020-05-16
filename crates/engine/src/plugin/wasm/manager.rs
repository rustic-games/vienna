use super::HandlerError;
use crate::error;
use crate::plugin::{wasm::Plugin, Handler, Runtime};
use common::{Event, GameState};
use std::{fmt, fs, path::Path};
use wasmtime::Store;

/// The object responsible for "managing" Wasm plugins.
#[derive(Default)]
pub struct Manager {
    /// The list of plugins this plugin manager is responsible for.
    plugins: Vec<Plugin>,

    /// The wasm cache used by the `wasmtime` Wasm runtime.
    plugin_store: Store,
}

impl fmt::Debug for Manager {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PluginManager")
            .field("plugins", &self.plugins)
            .field("plugin_store", &"wasmtime::Store".to_string())
            .finish()
    }
}

impl Handler for Manager {
    fn run_plugins(
        &mut self,
        game_state: &mut GameState,
        events: &[Event],
    ) -> Result<(), error::Runtime> {
        for plugin in &mut self.plugins {
            plugin.run(game_state, events)?;
        }

        Ok(())
    }

    fn register_plugin(
        &mut self,
        game_state: &mut GameState,
        file: &Path,
    ) -> Result<(), error::Handler> {
        let source = fs::read(file)
            .map_err(|err| (file.to_owned(), err))
            .map_err(HandlerError::from)?;

        let plugin = Plugin::new(&self.plugin_store, game_state, source)
            .map_err(|err| (file.to_owned(), err))
            .map_err(HandlerError::from)?;

        println!("plugin registered: {}", plugin.name());
        self.plugins.push(plugin);

        Ok(())
    }

    fn as_wasm(&mut self) -> Option<&mut Self> {
        Some(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use tempfile::NamedTempFile;

    mod run_plugins {
        use super::*;

        #[test]
        fn empty() {
            let mut game_state = GameState::default();
            let mut manager = Manager::default();

            assert!(manager.run_plugins(&mut game_state, &[]).is_ok())
        }

        #[test]
        fn multiple() {
            use crate::plugin::wasm::plugin::tests::WAT_VALID;
            let mut game_state = GameState::default();
            let mut manager = Manager::default();

            let p = plugin(WAT_VALID);
            manager.plugins.push(p);

            let p = plugin(WAT_VALID);
            manager.plugins.push(p);

            assert!(manager.run_plugins(&mut game_state, &[]).is_ok())
        }

        #[test]
        fn with_failure() {
            use crate::plugin::wasm::plugin::tests::{WAT_MISSING_FUNC, WAT_VALID};
            let mut game_state = GameState::default();
            let mut manager = Manager::default();

            let p = plugin(WAT_VALID);
            manager.plugins.push(p);

            let p = plugin(WAT_MISSING_FUNC);
            manager.plugins.push(p);

            let err = anyhow::Error::new(manager.run_plugins(&mut game_state, &[]).unwrap_err());

            assert_eq!(
                format!("{:?}", err),
                "wasm runtime error\n\n\

                 Caused by:\n    \
                     missing exported `_run` function"
            )
        }
    }

    mod register_plugin {
        use super::*;

        #[test]
        fn valid() {
            use crate::plugin::wasm::plugin::tests::WAT_VALID;
            let (_guard, path) = wasm(WAT_VALID);
            let mut game_state = GameState::default();

            assert!(Manager::default()
                .register_plugin(&mut game_state, &path)
                .is_ok())
        }

        #[test]
        fn invalid_wasm() {
            let (_guard, path) = wasm(r#"INVALID"#);
            let mut game_state = GameState::default();

            let result = Manager::default().register_plugin(&mut game_state, &path);
            let err = anyhow::Error::new(result.unwrap_err());

            assert_eq!(
                format!("{:?}", err),
                format!(
                    "wasm handler error\n\n\

                     Caused by:\n    \
                         0: invalid wasm module `{}`\n    \
                         1: unable to run module\n    \
                         2: expected `(`\n            \
                                 --> <anon>:1:1\n             \
                                  |\n           \
                                1 | INVALID\n             \
                                  | ^",
                    &path.to_string_lossy()
                )
            )
        }

        #[test]
        fn missing_file() {
            let path = "/missing/file";
            let mut game_state = GameState::default();

            let result = Manager::default().register_plugin(&mut game_state, Path::new(path));
            let err = anyhow::Error::new(result.unwrap_err());

            assert_eq!(
                format!("{:?}", err),
                "wasm handler error\n\n\

                 Caused by:\n    \
                     inaccessible wasm module `/missing/file` (NotFound)"
            )
        }
    }

    fn wasm(wasm: &str) -> (NamedTempFile, PathBuf) {
        use std::io::Write;

        let mut file = NamedTempFile::new().expect("temporary file");
        let path = file.path().to_owned();

        file.as_file_mut()
            .write_all(wasm.as_bytes())
            .expect("written bytes");

        (file, path)
    }

    fn plugin(wasm: &str) -> Plugin {
        let mut game_state = GameState::default();
        let store = Store::default();
        Plugin::new(&store, &mut game_state, wasm).unwrap()
    }
}
