// use super::{Error, Func, Plugin};
use super::RuntimeError;
use crate::plugin::{Func, Runtime};
use std::fmt;
use wasmtime::Instance;

/// A container type to wrap a Wasm module.
pub struct Plugin {
    /// The Wasm instance used to run the plugin logic.
    instance: Instance,
}

impl fmt::Debug for Plugin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Wasm")
            .field("instance", &"wasmtime::Instance".to_string())
            .finish()
    }
}

impl Runtime for Plugin {
    type Data = Instance;
    type Error = RuntimeError;

    /// Create a new plugin based on an existing Wasm instance.
    fn new(instance: Self::Data) -> Self {
        Self { instance }
    }

    /// Run the plugin.
    ///
    /// This requires the Wasm module to expose a `_run` function that takes
    /// zero arguments and returns no values.
    fn run(&self) -> Result<(), Self::Error> {
        let func = Func::Run;

        let run = self
            .instance
            .get_func(&func.to_string())
            .ok_or(Self::Error::MissingExportedFunction(func))?
            .get0::<()>()
            .map_err(|source| Self::Error::InvalidExportedFunction { func, source })?;

        run().map_err(|source| Self::Error::Failed { func, source })
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
