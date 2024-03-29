//! Internal implementation details used to run plugins.

// Since these internal methods are only used in the `load` macro, which is only
// used once per plugin, it makes sense to always inline them.
//
// Moreover, _somehow_ removing `always` from `fn run` breaks the code at
// runtime.
//
// see: https://discordapp.com/channels/442252698964721669/443151097398296587/712193675702042626
#![allow(clippy::inline_always)]

use crate::State;
use anyhow::Result;
use common::{serde_json, Registration, RunResult, StateTransfer};
use core::mem;
use std::convert::TryInto;

/// An internal function called by the `plugin!()` macro.
///
/// This function is called by the engine when a new plugin is loaded.
///
/// The `registration` attribute contains the details set by the plugin to be
/// used by the engine to validate the plugin registration.
#[inline(always)]
#[allow(clippy::match_wild_err_arm, clippy::as_conversions)]
pub fn init(registration: &Registration) {
    let data = match serde_json::to_vec(registration) {
        Ok(data) => data,
        Err(_) => todo!("logging"),
    };

    let mut slice = data.into_boxed_slice();
    let len = match slice.len().try_into() {
        Ok(len) => len,
        Err(_) => todo!("logging: struct too large"),
    };

    unsafe { ffi::init_callback(slice.as_mut_ptr() as i32, len) };
}

/// An internal function called by the `plugin!()` macro.
///
/// This function is called by the engine every time a plugin runs.
///
/// The `result` attribute contains any errors the plugin generated while
/// running.
#[inline(always)]
pub fn run(mut state: State, result: Result<()>) {
    let error = result.err().map(|err| format!("{:#}", err));

    // Populate the run result with the updated state, if any.
    let mut new_state = None;
    if state.updated {
        let mut state_transfer = StateTransfer::default();
        state_transfer.owned = mem::take(&mut state.owned);
        new_state = Some(state_transfer)
    }

    let run = RunResult {
        error,
        state: new_state,
    };

    let data = match serde_json::to_vec(&run) {
        Ok(vec) => vec,
        Err(err) => format!(r#"{{"error":"{:#}"}}"#, err).into_bytes(),
    };

    let mut slice = data.into_boxed_slice();

    #[allow(clippy::match_wild_err_arm)]
    let len = match slice.len().try_into() {
        Ok(len) => len,
        Err(_) => todo!("logging: struct too large"),
    };

    unsafe {
        #[allow(clippy::as_conversions)]
        ffi::run_callback(slice.as_mut_ptr() as i32, len)
    };
}

/// Allocate memory on the guest.
#[inline(always)]
#[must_use]
#[allow(clippy::match_wild_err_arm, clippy::as_conversions)]
pub fn malloc(len: i32) -> i32 {
    let vec = match len.try_into() {
        Ok(len) => Vec::<u8>::with_capacity(len),
        Err(_) => todo!("logging"),
    };

    mem::ManuallyDrop::new(vec).as_mut_ptr() as i32
}

/// Functions exposed by the engine for the plugins to call.
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
