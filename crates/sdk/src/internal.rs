use anyhow::Result;

/// An internal function called by the `load!()` macro.
///
/// This function is called by the engine every time a plugin runs.
///
/// The `result` attribute contains any errors the plugin generated while
/// running.
pub fn run(result: Result<()>) {
    match result {
        Ok(()) => {}
        Err(err) => panic!("{:?}", err),
    }
}
