//! Wasm-based plugin logic.

use super::RuntimeError;
use crate::{
    error,
    plugin::{Func, Runtime},
};
use common::{
    serde_json, Canvas, DeserializeOwned, Event, GameState, PluginState, Registration, RunResult,
    StateTransfer,
};
use std::cell::Cell;
use std::collections::HashMap;
use std::convert::TryInto;
use std::io::Write;
use std::rc::Rc;
use std::{fmt, mem};
use wasmtime::{Caller, Extern, Func as F, Instance, Memory, Module, Store, Trap, WasmTy};
use wasmtime_wasi::{Wasi, WasiCtx};

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
    /// Create a new wasm plugin.
    pub(super) fn new(
        store: &Store,
        game_state: &mut GameState,
        source: impl AsRef<[u8]>,
    ) -> Result<Self, RuntimeError> {
        let module = Module::new(store, source)?;

        let registration: Rc<Cell<Option<Registration>>> = Rc::new(Cell::new(None));
        let run_result = Rc::new(Cell::new(None));

        let mut host_functions = vec![
            Self::callback(store, Rc::clone(&registration)),
            Self::callback(store, Rc::clone(&run_result)),
        ];

        // TODO: limit what resources the modules have access to.
        #[allow(clippy::match_wild_err_arm)]
        let ctx = match WasiCtx::new(std::env::args()) {
            Ok(ctx) => ctx,
            Err(_) => todo!("logging"),
        };

        let wasi = Wasi::new(store, ctx);
        for import in module.imports() {
            if import.module() == "wasi_snapshot_preview1" {
                if let Some(export) = wasi.get_export(import.name()) {
                    host_functions.push(Extern::from(export.clone()));
                    continue;
                }
            }
        }

        let instance = Instance::new(&module, &host_functions)?;

        Self::call(&instance, Func::Init)?;

        let mut registration: Registration = match registration.take() {
            Some(registration) => registration,
            None => todo!("logging"),
        };

        if registration.name.is_empty() {
            return Err(RuntimeError::MissingName);
        }

        // Only register state plugin if anything needs to be tracked.
        let state = match &mut registration.state {
            Some(state) => mem::take(state),
            None => HashMap::default(),
        };

        let widgets = match &mut registration.widgets {
            Some(widgets) => mem::take(widgets),
            None => HashMap::default(),
        };

        let plugin_state = PluginState::new(state, widgets);

        game_state.register_plugin_state(registration.name.clone(), plugin_state);

        Ok(Self {
            instance,
            registration,
            run_result,
        })
    }

    /// Call into the wasm instance for a given function that takes no arguments.
    fn call(instance: &Instance, func: Func) -> Result<(), RuntimeError> {
        let call = instance
            .get_func(&func.to_string())
            .ok_or(RuntimeError::MissingExportedFunction(func))?
            .get0::<()>()
            .map_err(|source| RuntimeError::InvalidExportedFunction { func, source })?;

        call().map_err(|source| RuntimeError::Failed { func, source })?;

        Ok(())
    }

    /// Call into the wasm instance for a given function that takes one argument.
    fn call1<T: WasmTy>(instance: &Instance, func: Func, value: i32) -> Result<T, RuntimeError> {
        let call = instance
            .get_func(&func.to_string())
            .ok_or(RuntimeError::MissingExportedFunction(func))?
            .get1::<i32, T>()
            .map_err(|source| RuntimeError::InvalidExportedFunction { func, source })?;

        let value = call(value).map_err(|source| RuntimeError::Failed { func, source })?;

        Ok(value)
    }

    /// Call into the wasm instance for a given function that takes two arguments.
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

    /// An implementation of an engine function that is called by the plugin
    /// with a pointer at which the engine is expected to find a specific type.
    ///
    /// The function fetches that type, and stores it in a reference-counted
    /// cell.
    fn callback<T: std::fmt::Debug + DeserializeOwned + 'static>(
        store: &Store,
        ptr: Rc<Cell<Option<T>>>,
    ) -> Extern {
        F::wrap(store, move |caller: Caller<'_>, pos: i32, len: i32| {
            let mut memory = match Self::get_memory(&caller) {
                Ok(mem) => mem,
                Err(err) => return Err(Trap::new(err.to_string())),
            };

            // Safe, because we read the data and convert it to an owned type.
            // Plugin is not Send or Sync, so the memory cannot be accessed
            // concurrently.
            //
            // See: https://docs.rs/wasmtime/0.16.0/wasmtime/struct.Memory.html#memory-and-safety
            let data = unsafe {
                #[allow(clippy::as_conversions, clippy::cast_sign_loss)]
                let slice = get_data(&mut memory, pos as usize, len as usize);

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

    /// Get the live memory address of the wasm plugin instance.
    fn get_memory(caller: &Caller<'_>) -> Result<Memory, RuntimeError> {
        #[allow(clippy::match_wild_err_arm, clippy::wildcard_enum_match_arm)]
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
    fn run(
        &mut self,
        game_state: &mut GameState,
        canvas: Canvas,
        events: &[Event],
    ) -> Result<(), error::Runtime> {
        let owned = game_state.get(self.name()).cloned().unwrap_or_default();

        let mut borrowed = HashMap::default();
        if let Some(ref dependencies) = &self.registration.dependencies {
            for plugin in dependencies {
                if let Some(state) = game_state.get(plugin) {
                    borrowed.insert(plugin.clone(), state.clone());
                }
            }
        }

        let state = StateTransfer {
            owned,
            borrowed,
            canvas,
            events: events.to_vec(),
        };

        let vec = serde_json::to_vec(&state).map_err(RuntimeError::from)?;
        let vec_size: i32 = vec.len().try_into().map_err(RuntimeError::from)?;

        let offset: i32 = Self::call1(&self.instance, Func::Malloc, vec_size)?;
        let offset_size: usize = offset.try_into().map_err(RuntimeError::from)?;

        let mut memory = match self.instance.get_memory("memory") {
            Some(mem) => mem,
            None => todo!("logging"),
        };

        unsafe {
            let mut slice = get_data(&mut memory, offset_size, vec.len());

            if slice.write_all(&vec).is_err() {
                todo!("logging")
            }
        }

        Self::call2(&self.instance, Func::Run, offset, vec_size)?;

        let run = match self.run_result.take() {
            Some(run) => run,
            None => {
                // TODO: logging
                RunResult::default()
            }
        };

        if let Some(err) = run.error {
            return Err(RuntimeError::Plugin(err).into());
        }

        // If `state` is `None`, it means no state was changed by the plugin, so
        // the game state doesn't have to be updated.
        if let Some(mut state) = run.state {
            let StateTransfer { owned, .. } = mem::take(&mut state);
            game_state.register_plugin_state(self.name(), owned);
        }

        Ok(())
    }

    fn name(&self) -> &str {
        &self.registration.name
    }

    fn as_wasm(&mut self) -> Option<&mut Self> {
        Some(self)
    }
}

/// Given an instance of wasm memory, a position in that memory and the length
/// of the memory chunk, return whatever bytes are stored at this address.
///
/// # Safety
///
/// This expects all three provided values to be correct.
unsafe fn get_data(memory: &mut Memory, pos: usize, len: usize) -> &mut [u8] {
    let data = memory.data_unchecked_mut();

    #[allow(clippy::as_conversions, clippy::cast_sign_loss)]
    let total_len = match pos.checked_add(len) {
        Some(len) => len,
        None => todo!("logging"),
    };

    #[allow(clippy::cast_sign_loss, clippy::as_conversions)]
    match data.get_mut(pos..total_len) {
        Some(slice) => slice,
        None => todo!("logging"),
    }
}

#[cfg(test)]
#[allow(clippy::restriction)]
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
            let canvas = Canvas::default();
            let mut game_state = GameState::default();

            assert!(plugin(WAT_VALID)
                .expect("valid plugin")
                .run(&mut game_state, canvas, &[])
                .is_ok())
        }

        #[test]
        fn missing_function() {
            let canvas = Canvas::default();
            let mut game_state = GameState::default();
            let result =
                plugin(WAT_MISSING_FUNC)
                    .expect("valid plugin")
                    .run(&mut game_state, canvas, &[]);
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
            let canvas = Canvas::default();
            let mut game_state = GameState::default();
            let result = plugin(WAT_INVALID_FUNC_SIGNATURE)
                .expect("valid plugin")
                .run(&mut game_state, canvas, &[]);
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
            i32.const 12
            call $init_callback)
        (func (export "_run") (param i32 i32))
        (func (export "_malloc") (param i32) (result i32)
            i32.const 0)
        (data (;0;) (i32.const 1048576) "{\22n\22:\22test\22}")
        (memory (;0;) 17)
        (export "memory" (memory 0)))
    "#;

    // missing `_run` export
    pub const WAT_MISSING_FUNC: &str = r#"(module
        (import "" "init_callback" (func $init_callback (param i32 i32)))
        (import "" "run_callback" (func (param i32 i32)))
        (func (export "_init")
            i32.const 1048576
            i32.const 12
            call $init_callback)
        (func (export "_malloc") (param i32) (result i32)
            i32.const 0)
        (data (;0;) (i32.const 1048576) "{\22n\22:\22test\22}")
        (memory (;0;) 17)
        (export "memory" (memory 0)))
    "#;

    // invalid `_run` signature
    pub const WAT_INVALID_FUNC_SIGNATURE: &str = r#"(module
        (import "" "init_callback" (func $init_callback (param i32 i32)))
        (import "" "run_callback" (func (param i32 i32)))
        (func (export "_init")
            i32.const 1048576
            i32.const 12
            call $init_callback)
        (func (export "_run") (param i32 i32) (result i32)
            i32.const 42)
        (func (export "_malloc") (param i32) (result i32)
            i32.const 0)
        (data (;0;) (i32.const 1048576) "{\22n\22:\22test\22}")
        (memory (;0;) 17)
        (export "memory" (memory 0)))
    "#;
}
