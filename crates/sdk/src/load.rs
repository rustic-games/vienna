#[macro_export]
macro_rules! load {
    () => {
        use $crate::prelude::*;

        #[no_mangle]
        /// Run the plugin on every game update.
        pub extern "C" fn _run() {
            // Explicit type to improve compiler error for plugin authors.
            let result: Result<()> = run();

            $crate::run(result);
        }
    };
}
