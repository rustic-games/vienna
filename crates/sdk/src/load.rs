#[macro_export]
macro_rules! load {
    () => {
        use $crate::prelude::*;

        #[no_mangle]
        /// Run the plugin on every game update.
        pub extern "C" fn _run() {
            match run() {
                Ok(()) => {}
                Err(err) => panic!("{:?}", err),
            }
        }
    };
}
