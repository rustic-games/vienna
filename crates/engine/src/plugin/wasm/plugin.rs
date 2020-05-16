use super::RuntimeError;
use crate::error;
use crate::plugin::{Func, Runtime};
use common::{GameState, Registration, RunResult, State};
use std::cell::Cell;
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;
use std::io::Write;
use std::rc::Rc;
use wasmtime::{Caller, Extern, Func as F, Instance, Memory, Module, Store, Trap, WasmTy};

/// A container type to wrap a Wasm module.
pub struct Plugin {
    /// The Wasm instance used to run the plugin logic.
    instance: Instance,

    /// Registration details exposed by the Wasm instance.
    registration: Registration,

    /// The result of a run.
    ///
    /// This value can't actually be accessed, it merely serves as a conduit
    /// between the Wasm instance and the `Plugin::run` method:
    ///
    /// 1. On initialization, a callback function is created which takes a
    ///    reference-counted pointer to this value.
    ///
    /// 2. When the plugin finishes running, it stores its results in this
    ///    object.
    ///
    /// 3. The `Plugin::run` method then takes this value and uses its results,
    ///    leaving `None` in its place.
    run_result: Rc<Cell<Option<RunResult>>>,
}

impl Plugin {
    pub(super) fn new(
        store: &Store,
        game_state: &mut GameState,
        source: impl AsRef<[u8]>,
    ) -> Result<Self, RuntimeError> {
        let module = Module::new(store, source)?;

        let registration = Rc::new(Cell::new(None));
        let run_result = Rc::new(Cell::new(None));

        let host_functions = vec![
            Self::callback(store, Rc::clone(&registration)),
            Self::callback(store, Rc::clone(&run_result)),
        ];
        let instance = Instance::new(&module, &host_functions)?;

        Self::call(&instance, Func::Init)?;

        let registration: Registration = registration.take().unwrap_or_default();

        if registration.name.is_empty() {
            return Err(RuntimeError::MissingName);
        }

        game_state.register_plugin_state(registration.name.clone(), registration.write.clone());

        Ok(Self {
            instance,
            registration,
            run_result,
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

    fn call1<T: WasmTy>(instance: &Instance, func: Func, value: i32) -> Result<T, RuntimeError> {
        let call = instance
            .get_func(&func.to_string())
            .ok_or(RuntimeError::MissingExportedFunction(func))?
            .get1::<i32, T>()
            .map_err(|source| RuntimeError::InvalidExportedFunction { func, source })?;

        let value = call(value).map_err(|source| RuntimeError::Failed { func, source })?;

        Ok(value)
    }

    fn call2<T: WasmTy>(
        instance: &Instance,
        func: Func,
        v1: i32,
        v2: i32,
    ) -> Result<T, RuntimeError> {
        let call = instance
            .get_func(&func.to_string())
            .ok_or(RuntimeError::MissingExportedFunction(func))?
            .get2::<i32, i32, T>()
            .map_err(|source| RuntimeError::InvalidExportedFunction { func, source })?;

        let value = call(v1, v2).map_err(|source| RuntimeError::Failed { func, source })?;

        Ok(value)
    }

    fn callback<T: std::fmt::Debug + serde::de::DeserializeOwned + 'static>(
        store: &Store,
        ptr: Rc<Cell<Option<T>>>,
    ) -> Extern {
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
            let data = unsafe {
                let data = memory.data_unchecked();

                #[allow(clippy::cast_sign_loss)]
                let slice = &data[pos as usize..(len as usize + pos as usize)];

                match serde_json::from_slice(slice) {
                    Ok(value) => value,
                    Err(err) => return Err(Trap::new(err.to_string())),
                }
            };

            ptr.replace(Some(data));

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
    /// This requires the Wasm module to expose a `_run` function that takes two
    /// i32 arguments and returns no values.
    fn run(&mut self, game_state: &mut GameState) -> Result<(), error::Runtime> {
        let mut borrowed = HashMap::default();
        for (plugin, keys) in self.registration.read.clone() {
            match game_state.borrowed_state(plugin.clone(), &keys) {
                None => {}
                Some(state) => {
                    borrowed.insert(plugin, state);
                }
            }
        }

        // Take the state owned by the plugin. If the plugin has requested no
        // state to maintain, a default empty one is created instead.
        let owned = game_state
            .owned_state(self.name())
            .cloned()
            .unwrap_or_default();

        let state = State::new(owned, borrowed);
        let vec = serde_json::to_vec(&state).map_err(RuntimeError::from)?;
        let vec_size: i32 = vec.len().try_into().map_err(RuntimeError::from)?;

        let offset: i32 = Self::call1(&self.instance, Func::Malloc, vec_size)?;
        let offset_size: usize = offset.try_into().map_err(RuntimeError::from)?;

        let memory = self.instance.get_memory("memory").unwrap();

        unsafe {
            let data = memory.data_unchecked_mut();
            let mut slice = &mut data[offset_size..(offset_size + vec.len())];
            slice.write_all(&vec).unwrap();
        }

        Self::call2(&self.instance, Func::Run, offset, vec_size)?;

        let mut run = match self.run_result.take() {
            Some(run) => run,
            None => {
                // TODO: logging
                RunResult::default()
            }
        };

        if let Some(err) = run.error {
            return Err(RuntimeError::Plugin(err).into());
        }

        let (state, _) = std::mem::take(&mut run.state).into_parts();
        game_state.replace_plugin_state(self.name(), state);

        Ok(())
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
                "unable to run module\n\n\

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
            let mut game_state = GameState::default();

            assert!(plugin(WAT_VALID)
                .expect("valid plugin")
                .run(&mut game_state)
                .is_ok())
        }

        #[test]
        fn missing_function() {
            let mut game_state = GameState::default();
            let result = plugin(WAT_MISSING_FUNC)
                .expect("valid plugin")
                .run(&mut game_state);
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
            let mut game_state = GameState::default();
            let result = plugin(WAT_INVALID_FUNC_SIGNATURE)
                .expect("valid plugin")
                .run(&mut game_state);
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
        let mut game_state = GameState::default();
        let store = wasmtime::Store::default();
        Plugin::new(&store, &mut game_state, wasm)
    }

    pub const WAT_VALID: &str = r#"(module
        (import "" "init_callback" (func $init_callback (param i32 i32)))
        (import "" "run_callback" (func (param i32 i32)))
        (func (export "_init")
            i32.const 1048576
            i32.const 36
            call $init_callback)
        (func (export "_run") (param i32 i32))
        (func (export "_malloc") (param i32) (result i32)
            i32.const 0)
        (data (;0;) (i32.const 1048576) "{\22name\22:\22test\22,\22write\22:{},\22read\22:{}}")
        (memory (;0;) 17)
        (export "memory" (memory 0)))
    "#;

    // missing `_run` export
    pub const WAT_MISSING_FUNC: &str = r#"(module
        (import "" "init_callback" (func $init_callback (param i32 i32)))
        (import "" "run_callback" (func (param i32 i32)))
        (func (export "_init")
            i32.const 1048576
            i32.const 36
            call $init_callback)
        (func (export "_malloc") (param i32) (result i32)
            i32.const 0)
        (data (;0;) (i32.const 1048576) "{\22name\22:\22test\22,\22write\22:{},\22read\22:{}}")
        (memory (;0;) 17)
        (export "memory" (memory 0)))
    "#;

    // invalid `_run` signature
    pub const WAT_INVALID_FUNC_SIGNATURE: &str = r#"(module
        (import "" "init_callback" (func $init_callback (param i32 i32)))
        (import "" "run_callback" (func (param i32 i32)))
        (func (export "_init")
            i32.const 1048576
            i32.const 36
            call $init_callback)
        (func (export "_run") (param i32 i32) (result i32)
            i32.const 42)
        (func (export "_malloc") (param i32) (result i32)
            i32.const 0)
        (data (;0;) (i32.const 1048576) "{\22name\22:\22test\22,\22write\22:{},\22read\22:{}}")
        (memory (;0;) 17)
        (export "memory" (memory 0)))
    "#;
}
