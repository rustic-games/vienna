use super::RuntimeError;
use crate::error;
use crate::plugin::{Func, Runtime};
use std::fmt;
use wasmtime::{Instance, Module, Store};

/// A container type to wrap a Wasm module.
pub struct Plugin {
    /// The Wasm instance used to run the plugin logic.
    instance: Instance,
}

impl Plugin {
    pub(super) fn new(store: &Store, source: impl AsRef<[u8]>) -> Result<Self, RuntimeError> {
        let module = Module::new(store, source)?;
        let instance = Instance::new(&module, &[])?;

        Ok(Self { instance })
    }
}

impl fmt::Debug for Plugin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Wasm")
            .field("instance", &"wasmtime::Instance".to_string())
            .finish()
    }
}

impl Runtime for Plugin {
    /// Run the plugin.
    ///
    /// This requires the Wasm module to expose a `_run` function that takes
    /// zero arguments and returns no values.
    fn run(&mut self) -> Result<(), error::Runtime> {
        let func = Func::Run;

        let run = self
            .instance
            .get_func(&func.to_string())
            .ok_or(RuntimeError::MissingExportedFunction(func))?
            .get0::<()>()
            .map_err(|source| RuntimeError::InvalidExportedFunction { func, source })?;

        run().map_err(|source| RuntimeError::Failed { func, source })?;

        Ok(())
    }

    fn as_wasm(&mut self) -> Option<&mut Self> {
        Some(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod new {
        use super::*;

        #[test]
        fn valid() {
            let wasm = r#"(module (func (export "_run")))"#;

            assert!(plugin(wasm).is_ok())
        }

        #[test]
        fn invalid_wasm() {
            let wasm = "INVALID";

            let err = anyhow::Error::new(plugin(wasm).unwrap_err());

            assert_eq!(
                format!("{:?}", err),
                "invalid wasm module\n\n\

                 Caused by:\n    \
                     expected `(`\n         \
                          --> <anon>:1:1\n          \
                        |\n        \
                      1 | INVALID\n          \
                        | ^"
            )
        }
    }

    mod run {
        use super::*;

        #[test]
        fn valid() {
            let wasm = r#"(module (func (export "_run")))"#;

            assert!(plugin(wasm).expect("valid plugin").run().is_ok())
        }

        #[test]
        fn missing_function() {
            let wasm = r#"(module (func (export "_invalid")))"#;

            let result = plugin(wasm).expect("valid plugin").run();
            let err = anyhow::Error::new(result.unwrap_err());

            assert_eq!(
                format!("{:?}", err),
                "wasm runtime error\n\n\

                 Caused by:\n    \
                     missing exported `_run` function"
            )
        }

        #[test]
        fn invalid_function_signature() {
            let wasm = r#"(module (func (export "_run") (result i32) i32.const 42))"#;

            let result = plugin(wasm).expect("valid plugin").run();
            let err = anyhow::Error::new(result.unwrap_err());

            assert_eq!(
                format!("{:?}", err),
                "wasm runtime error\n\n\

                 Caused by:\n    \
                     0: invalid exported `_run` function\n    \
                     1: Type mismatch: too many return values (expected 1)"
            )
        }
    }

    fn plugin(wasm: &str) -> Result<Plugin, RuntimeError> {
        let store = wasmtime::Store::default();
        Plugin::new(&store, wasm)
    }
}
