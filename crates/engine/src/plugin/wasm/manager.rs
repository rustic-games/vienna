use super::plugin::Plugin;
use super::HandlerError;
use crate::plugin::{Handler, Runtime};
use std::fmt;
use wasmtime::{Instance, Module, Store};

/// The object responsible for "managing" Wasm plugins.
#[derive(Default)]
pub struct Manager {
    /// The list of plugins this plugin manager is responsible for.
    plugins: Vec<Plugin>,

    // The wasm cache used by the `wasmtime` Wasm runtime.
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
    type Error = HandlerError;

    fn run_plugins(&mut self) -> Result<(), Self::Error> {
        for plugin in &self.plugins {
            plugin.run()?
        }

        Ok(())
    }

    fn register_plugin(&mut self, path: &str) -> Result<(), Self::Error> {
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
    use crate::plugin::Wasm;
    use tempfile::NamedTempFile;

    mod run_plugins {
        use super::*;

        #[test]
        fn empty() {
            let mut manager = Manager::default();

            assert!(manager.run_plugins().is_ok())
        }

        #[test]
        fn multiple() {
            let mut manager = Manager::default();

            let p = plugin(r#"(module (func (export "_run")))"#);
            manager.plugins.push(p);

            let p = plugin(r#"(module (func (export "_run")))"#);
            manager.plugins.push(p);

            assert!(manager.run_plugins().is_ok())
        }

        #[test]
        fn with_failure() {
            let mut manager = Manager::default();

            let p = plugin(r#"(module (func (export "_run")))"#);
            manager.plugins.push(p);

            let p = plugin(r#"(module (func (export "INVALID")))"#);
            manager.plugins.push(p);

            let err = manager.run_plugins().unwrap_err();

            assert_eq!(err.to_string(), format!("error running wasm instance"))
        }
    }

    mod register_plugin {
        use super::*;

        #[test]
        fn valid() {
            let (_guard, path) = wasm(r#"(module (func (export "_run")))"#);

            assert!(Manager::default().register_plugin(&path).is_ok())
        }

        #[test]
        fn invalid_wasm() {
            let (_guard, path) = wasm(r#"INVALID"#);

            let err = Manager::default().register_plugin(&path).unwrap_err();

            assert_eq!(err.to_string(), format!("invalid wasm module `{}`", path))
        }

        #[test]
        fn missing_file() {
            let path = "/missing/file";

            let err = Manager::default().register_plugin(&path).unwrap_err();

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

    fn plugin(wasm: &str) -> Wasm {
        let store = wasmtime::Store::default();
        let module = Module::new(&store, wasm).unwrap();
        let instance = Instance::new(&module, &[]).unwrap();

        Wasm::new(instance)
    }
}
