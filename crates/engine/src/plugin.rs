use crate::error::PluginError as Error;
use displaydoc::Display;
use wasmtime::Instance;

type Result<T> = std::result::Result<T, Error>;

/// A list of exported functions the engine expects a plugin to have.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq, Display)]
pub enum Func {
    /// _run
    Run,
}

/// A container type to wrap a Wasm module.
pub(crate) struct Plugin {
    /// The Wasm instance used to run the plugin logic.
    instance: Instance,
}

impl Plugin {
    /// Create a new plugin based on an existing Wasm instance.
    pub(crate) fn new(instance: Instance) -> Self {
        Plugin { instance }
    }

    /// Run the plugin.
    ///
    /// This requires the Wasm module to expose a `_run` function that takes
    /// zero arguments and returns no values.
    pub(crate) fn run(&self) -> Result<()> {
        let func = Func::Run;

        let run = self
            .instance
            .get_func(&func.to_string())
            .ok_or(Error::MissingExportedFunction(func))?
            .get0::<()>()
            .map_err(|source| Error::InvalidExportedFunction { func, source })?;

        run().map_err(|source| Error::RuntimeError { func, source })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod run {
        use super::*;

        #[test]
        fn valid() {
            let wasm = r#"(module (func (export "_run")))"#;

            assert!(plugin(wasm).run().is_ok())
        }

        #[test]
        fn missing_function() {
            let wasm = r#"(module (func (export "_invalid")))"#;

            assert_eq!(
                plugin(wasm).run().unwrap_err().to_string(),
                format!("missing exported `{}` function", Func::Run)
            )
        }

        #[test]
        fn invalid_function_signature() {
            let wasm = r#"(module (func (export "_run") (result i32) i32.const 42))"#;

            assert_eq!(
                plugin(wasm).run().unwrap_err().to_string(),
                format!("invalid exported `{}` function", Func::Run)
            )
        }
    }

    fn plugin(wasm: &str) -> Plugin {
        use wasmtime::Module;

        let store = wasmtime::Store::default();
        let module = Module::new(&store, wasm).unwrap();
        let instance = Instance::new(&module, &[]).unwrap();

        Plugin::new(instance)
    }
}
