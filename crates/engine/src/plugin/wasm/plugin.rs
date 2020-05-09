use super::RuntimeError;
use crate::error;
use crate::plugin::{Func, Runtime};
use std::cell::Cell;
use std::fmt;
use std::rc::Rc;
use wasmtime::{Caller, Extern, Func as F, Instance, Memory, Module, Store, Trap};

/// A container type to wrap a Wasm module.
pub struct Plugin {
    /// The Wasm instance used to run the plugin logic.
    instance: Instance,

    /// Details about the plugin after registration.
    registration: vienna::Registration,
}

impl Plugin {
    pub(super) fn new(store: &Store, source: impl AsRef<[u8]>) -> Result<Self, RuntimeError> {
        let module = Module::new(store, source)?;
        let registration = Rc::new(Cell::new(None));
        let ptr = Rc::clone(&registration);
        let host_functions = &[Self::init_callback(store, ptr), Self::run_callback(store)];
        let instance = Instance::new(&module, host_functions)?;

        Self::call(&instance, Func::Init)?;

        let registration = match registration.take() {
            Some(registration) => registration,
            None => return Err(RuntimeError::Registration),
        };

        Ok(Self {
            instance,
            registration,
        })
    }

    fn call(instance: &Instance, func: Func) -> Result<(), RuntimeError> {
        let call = instance
            .get_func(&func.to_string())
            .ok_or(RuntimeError::MissingExportedFunction(func))?
            .get0::<()>()
            .map_err(|source| RuntimeError::InvalidExportedFunction { func, source })?;

        call().map_err(|source| RuntimeError::Failed { func, source })?;

        Ok(())
    }

    fn run_callback(store: &Store) -> Extern {
        F::wrap(store, |_: Caller<'_>, _: i32, _: i32| {}).into()
    }

    fn init_callback(store: &Store, ptr: Rc<Cell<Option<vienna::Registration>>>) -> Extern {
        F::wrap(store, move |caller: Caller<'_>, pos: i32, len: i32| {
            let memory = match Self::get_memory(&caller) {
                Ok(mem) => mem,
                Err(err) => return Err(Trap::new(err.to_string())),
            };

            // Safe, because we read the data and convert it to an owned type.
            // Plugin is not Send or Sync, so the memory cannot be accessed
            // concurrently.
            //
            // See: https://docs.rs/wasmtime/0.16.0/wasmtime/struct.Memory.html#memory-and-safety
            let registration = unsafe {
                let data = memory.data_unchecked();

                #[allow(clippy::cast_sign_loss)]
                let slice = &data[pos as usize..(len as usize + pos as usize)];

                match serde_json::from_slice(slice) {
                    Ok(value) => value,
                    Err(err) => return Err(Trap::new(err.to_string())),
                }
            };

            ptr.replace(Some(registration));

            Ok(())
        })
        .into()
    }

    fn get_memory(caller: &Caller<'_>) -> Result<Memory, RuntimeError> {
        match caller.get_export("memory") {
            Some(Extern::Memory(mem)) => Ok(mem),
            _ => Err(RuntimeError::MemoryAccess),
        }
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
        Self::call(&self.instance, Func::Run).map_err(Into::into)
    }

    fn name(&self) -> &str {
        &self.registration.name
    }

    fn as_wasm(&mut self) -> Option<&mut Self> {
        Some(self)
    }
}

#[cfg(test)]
pub(super) mod tests {
    use super::*;

    mod new {
        use super::*;

        #[test]
        fn valid() {
            assert!(plugin(WAT_VALID).is_ok())
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
            assert!(plugin(WAT_VALID).expect("valid plugin").run().is_ok())
        }

        #[test]
        fn missing_function() {
            let result = plugin(WAT_MISSING_FUNC).expect("valid plugin").run();
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
            let result = plugin(WAT_INVALID_FUNC_SIGNATURE)
                .expect("valid plugin")
                .run();
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

    #[test]
    fn name() {
        assert_eq!(plugin(WAT_VALID).expect("valid plugin").name(), "test")
    }

    fn plugin(wasm: &str) -> Result<Plugin, RuntimeError> {
        let store = wasmtime::Store::default();
        Plugin::new(&store, wasm)
    }

    pub const WAT_VALID: &str = r#"(module
        (import "" "init_callback" (func $init_callback (param i32 i32)))
        (import "" "run_callback" (func (param i32 i32)))
        (func (export "_init")
            i32.const 1048576
            i32.const 15
            call $init_callback)
        (func (export "_run"))
        (data (;0;) (i32.const 1048576) "{\22name\22:\22test\22}")
        (memory (;0;) 17)
        (export "memory" (memory 0)))
    "#;

    // missing `_run` export
    pub const WAT_MISSING_FUNC: &str = r#"(module
        (import "" "init_callback" (func $init_callback (param i32 i32)))
        (import "" "run_callback" (func (param i32 i32)))
        (func (export "_init")
            i32.const 1048576
            i32.const 15
            call $init_callback)
        (data (;0;) (i32.const 1048576) "{\22name\22:\22test\22}")
        (memory (;0;) 17)
        (export "memory" (memory 0)))
    "#;

    // invalid `_run` signature
    pub const WAT_INVALID_FUNC_SIGNATURE: &str = r#"(module
        (import "" "init_callback" (func $init_callback (param i32 i32)))
        (import "" "run_callback" (func (param i32 i32)))
        (func (export "_init")
            i32.const 1048576
            i32.const 15
            call $init_callback)
        (func (export "_run") (result i32)
            i32.const 42)
        (data (;0;) (i32.const 1048576) "{\22name\22:\22test\22}")
        (memory (;0;) 17)
        (export "memory" (memory 0)))
    "#;
}
