use crate::Sdk;
use anyhow::Result;
use common::{serde_json, Registration, RunResult, StateTransfer};
use core::mem;

/// An internal function called by the `load!()` macro.
///
/// This function is called by the engine when a new plugin is loaded.
///
/// The `registration` attribute contains the details set by the plugin to be
/// used by the engine to validate the plugin registration.
#[inline(always)]
pub fn init(registration: Registration) {
    let data = match serde_json::to_vec(&registration) {
        Ok(vec) => vec,
        Err(_) => return,
    };

    let mut slice = data.into_boxed_slice();

    unsafe { ffi::init_callback(slice.as_mut_ptr() as i32, slice.len() as i32) };
}

/// An internal function called by the `load!()` macro.
///
/// This function is called by the engine every time a plugin runs.
///
/// The `result` attribute contains any errors the plugin generated while
/// running.
#[inline(always)]
pub fn run(mut sdk: Sdk, result: Result<()>) {
    let error = result.err().map(|err| format!("{:#}", err));

    // Populate the run result with the updated state, if any.
    let mut state = None;
    if sdk.state_updated {
        let mut state_transfer = StateTransfer::default();
        state_transfer.owned = mem::take(&mut sdk.owned_state);
        state = Some(state_transfer)
    }

    let run = RunResult { error, state };
    let data = match serde_json::to_vec(&run) {
        Ok(vec) => vec,
        Err(err) => format!(r#"{{"error":"{:#}"}}"#, err).into_bytes(),
    };

    let mut slice = data.into_boxed_slice();
    unsafe { ffi::run_callback(slice.as_mut_ptr() as i32, slice.len() as i32) };
}

/// Allocate memory on the guest.
#[inline(always)]
pub fn malloc(len: i32) -> i32 {
    let vec = Vec::<u8>::with_capacity(len as usize);
    mem::ManuallyDrop::new(vec).as_mut_ptr() as i32
}

pub mod ffi {
    #[link(wasm_import_module = "")]
    extern "C" {
        /// Call back to the engine with a pointer and memory length to indicate
        /// where the `Registration` data can be found in the plugin's memory.
        pub fn init_callback(ptr: i32, len: i32);

        /// Call back to the engine with a pointer and memory length to indicate
        /// where the error data can be found in the plugin's memory.
        pub fn run_callback(ptr: i32, len: i32);
    }
}
