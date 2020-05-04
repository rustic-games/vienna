use crate::error::PluginManagerError as Error;
use crate::plugin::Plugin;

type Result<T> = std::result::Result<T, Error>;

/// The object responsible for "managing" engine plugins.
///
/// The responsibility of this manager includes:
///
/// - Loading new Wasm-based plugins.
/// - Running plugins when requested.
#[derive(Default)]
pub(crate) struct PluginManager {
    /// The list of plugins this plugin manager is responsible for.
    plugins: Vec<Plugin>,

    // The wasm cache used by `wasmtime`.
    plugin_store: wasmtime::Store,
}

impl PluginManager {
    /// Run all registered plugins.
    ///
    /// If a plugin fails, the manager stops running any pending plugins and
    /// returns an error.
    pub(crate) fn run_plugins(&self) -> Result<()> {
        for plugin in &self.plugins {
            plugin.run()?
        }

        Ok(())
    }

    /// Register a new plugin for this manager to manage.
    ///
    /// The provided path has to point to a valid Wasm file for this method to
    /// succeed.
    pub(crate) fn register_plugin(&mut self, path: &str) -> Result<()> {
        use wasmtime::{Instance, Module};

        let module = Module::from_file(&self.plugin_store, path).map_err(|err| (path, err))?;
        let instance = Instance::new(&module, &[]).map_err(|err| (path, err))?;
        let plugin = Plugin::new(instance);

        self.plugins.push(plugin);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    mod run_plugins {
        use super::*;

        #[test]
        fn empty() {
            let manager = PluginManager::default();

            assert!(manager.run_plugins().is_ok())
        }

        #[test]
        fn multiple() {
            let mut manager = PluginManager::default();

            let p = plugin(r#"(module (func (export "_run")))"#);
            manager.plugins.push(p);

            let p = plugin(r#"(module (func (export "_run")))"#);
            manager.plugins.push(p);

            assert!(manager.run_plugins().is_ok())
        }

        #[test]
        fn with_failure() {
            let mut manager = PluginManager::default();

            let p = plugin(r#"(module (func (export "_run")))"#);
            manager.plugins.push(p);

            let p = plugin(r#"(module (func (export "INVALID")))"#);
            manager.plugins.push(p);

            let err = manager.run_plugins().unwrap_err();

            assert_eq!(err.to_string(), format!("error running plugin"))
        }
    }

    mod register_plugin {
        use super::*;

        #[test]
        fn valid() {
            let (_guard, path) = wasm(r#"(module (func (export "_run")))"#);

            assert!(PluginManager::default().register_plugin(&path).is_ok())
        }

        #[test]
        fn invalid_wasm() {
            let (_guard, path) = wasm(r#"INVALID"#);

            let err = PluginManager::default().register_plugin(&path).unwrap_err();

            assert_eq!(err.to_string(), format!("invalid wasm module `{}`", path))
        }

        #[test]
        fn missing_file() {
            let path = "/missing/file";

            let err = PluginManager::default().register_plugin(&path).unwrap_err();

            assert_eq!(
                err.to_string(),
                format!("inaccessible wasm module `{}` (NotFound)", path)
            )
        }
    }

    fn wasm(wasm: &str) -> (NamedTempFile, String) {
        use std::io::Write;

        let mut file = NamedTempFile::new().expect("temporary file");
        file.as_file_mut()
            .write_all(wasm.as_bytes())
            .expect("written bytes");

        let path = file.path().to_str().expect("valid path").to_owned();

        (file, path)
    }

    fn plugin(wasm: &str) -> Plugin {
        use wasmtime::{Instance, Module};

        let store = wasmtime::Store::default();
        let module = Module::new(&store, wasm).unwrap();
        let instance = Instance::new(&module, &[]).unwrap();

        Plugin::new(instance)
    }
}
