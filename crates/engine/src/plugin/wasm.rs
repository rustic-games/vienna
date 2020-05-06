use super::{Error, Func, Plugin, Result};
use std::fmt;
use wasmtime::Instance;

/// A container type to wrap a Wasm module.
pub struct Wasm {
    /// The Wasm instance used to run the plugin logic.
    instance: Instance,
}

impl fmt::Debug for Wasm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Wasm")
            .field("instance", &"wasmtime::Instance".to_string())
            .finish()
    }
}

impl Plugin for Wasm {
    type Error = Error;

    /// Create a new plugin based on an existing Wasm instance.
    fn new(instance: Instance) -> Self {
        Self { instance }
    }

    /// Run the plugin.
    ///
    /// This requires the Wasm module to expose a `_run` function that takes
    /// zero arguments and returns no values.
    fn run(&self) -> Result<()> {
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

    fn plugin(wasm: &str) -> Wasm {
        use wasmtime::Module;

        let store = wasmtime::Store::default();
        let module = Module::new(&store, wasm).unwrap();
        let instance = Instance::new(&module, &[]).unwrap();

        Wasm::new(instance)
    }
}